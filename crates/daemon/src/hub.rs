use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use vortex_core::model::{OtpPopupEvent, constant_time_eq, generate_random_bytes_32, hex_encode};
use vortex_core::store::PairedDevice;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairingTicket {
    pub device_id: String,
    pub device_name: String,
    pub one_time_token: String,
    pub shared_secret: String,
    pub pairing_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedEnvelope {
    pub device_id: String,
    pub nonce: u64,
    pub ciphertext_b64: String,
    pub mac_hex: String,
}

pub fn create_pairing_ticket(
    hub_addr: &str,
    device_id: &str,
    device_name: &str,
    seed: u64,
) -> (PairingTicket, PairedDevice) {
    let entropy = generate_random_bytes_32(b"vortex-hub-pairing", seed);
    let mut secret_hasher = Sha256::new();
    secret_hasher.update(b"vortex-hub-shared-secret:");
    secret_hasher.update(device_id.as_bytes());
    secret_hasher.update(seed.to_le_bytes());
    secret_hasher.update(entropy);
    let shared_secret = hex_encode(&secret_hasher.finalize());

    let mut token_hasher = Sha256::new();
    token_hasher.update(b"vortex-one-time-token:");
    token_hasher.update(shared_secret.as_bytes());
    token_hasher.update(entropy);
    let one_time_token = hex_encode(&token_hasher.finalize()[..8]);

    let shared_secret_hash = hash_shared_secret(&shared_secret);
    let pairing_uri = format!(
        "vortex-pair://{hub_addr}?device={device_id}&token={one_time_token}&secret={shared_secret}"
    );

    let ticket = PairingTicket {
        device_id: device_id.to_string(),
        device_name: device_name.to_string(),
        one_time_token,
        shared_secret,
        pairing_uri,
    };
    let record = PairedDevice {
        device_id: device_id.to_string(),
        device_name: device_name.to_string(),
        shared_secret_hash,
        paired_at: seed as i64,
        revoked: false,
    };
    (ticket, record)
}

pub fn hash_shared_secret(secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"vortex-paired-secret-hash:");
    hasher.update(secret.as_bytes());
    hex_encode(&hasher.finalize())
}

pub fn encrypt_e2e_payload(
    device_id: &str,
    shared_secret: &str,
    nonce: u64,
    plaintext_json: &str,
) -> EncryptedEnvelope {
    let cipher = stream_xor(shared_secret.as_bytes(), nonce, plaintext_json.as_bytes());
    let ciphertext_b64 = base64::engine::general_purpose::STANDARD.encode(&cipher);
    let mac_hex = compute_envelope_mac(device_id, shared_secret, nonce, &ciphertext_b64);
    EncryptedEnvelope {
        device_id: device_id.to_string(),
        nonce,
        ciphertext_b64,
        mac_hex,
    }
}

pub fn decrypt_e2e_payload(
    shared_secret: &str,
    envelope: &EncryptedEnvelope,
) -> Result<String, String> {
    let expected_mac = compute_envelope_mac(
        &envelope.device_id,
        shared_secret,
        envelope.nonce,
        &envelope.ciphertext_b64,
    );
    let actual_mac = envelope.mac_hex.trim().to_ascii_lowercase();
    if !constant_time_eq(expected_mac.as_bytes(), actual_mac.as_bytes()) {
        return Err("E2E envelope MAC verification failed".into());
    }
    let cipher = base64::engine::general_purpose::STANDARD
        .decode(&envelope.ciphertext_b64)
        .map_err(|e| format!("invalid base64 ciphertext: {e}"))?;
    let plain = stream_xor(shared_secret.as_bytes(), envelope.nonce, &cipher);
    String::from_utf8(plain).map_err(|e| format!("invalid utf8 plaintext: {e}"))
}

fn compute_envelope_mac(
    device_id: &str,
    shared_secret: &str,
    nonce: u64,
    ciphertext_b64: &str,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"vortex-e2e-mac-v1:");
    hasher.update(device_id.as_bytes());
    hasher.update(b":");
    hasher.update(shared_secret.as_bytes());
    hasher.update(b":");
    hasher.update(nonce.to_le_bytes());
    hasher.update(b":");
    hasher.update(ciphertext_b64.as_bytes());
    hex_encode(&hasher.finalize())
}

fn stream_xor(secret: &[u8], nonce: u64, input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    for (block_idx, chunk) in input.chunks(32).enumerate() {
        let mut hasher = Sha256::new();
        hasher.update(b"vortex-e2e-stream-v1:");
        hasher.update(secret);
        hasher.update(nonce.to_le_bytes());
        hasher.update((block_idx as u64).to_le_bytes());
        let keystream = hasher.finalize();
        for (b, k) in chunk.iter().zip(keystream.iter()) {
            out.push(b ^ k);
        }
    }
    out
}

#[derive(Default)]
pub struct HubBroadcaster {
    seen_code_hashes: HashSet<String>,
    nonce_counter: u64,
}

impl HubBroadcaster {
    pub fn new() -> Self {
        let seed_bytes = generate_random_bytes_32(b"vortex-hub-nonce-init", 0);
        let mut init_nonce = [0u8; 8];
        init_nonce.copy_from_slice(&seed_bytes[..8]);
        Self {
            seen_code_hashes: HashSet::new(),
            nonce_counter: u64::from_le_bytes(init_nonce),
        }
    }

    pub fn fanout_otp_event(
        &mut self,
        event: &OtpPopupEvent,
        devices_and_secrets: &[(PairedDevice, String)],
    ) -> Vec<EncryptedEnvelope> {
        if !self.seen_code_hashes.insert(event.code_hash.clone()) {
            return Vec::new();
        }
        let Ok(json) = serde_json::to_string(event) else {
            return Vec::new();
        };

        let mut envelopes = Vec::new();
        for (dev, secret) in devices_and_secrets {
            if dev.revoked {
                continue;
            }
            if hash_shared_secret(secret) != dev.shared_secret_hash {
                continue;
            }
            self.nonce_counter = self.nonce_counter.wrapping_add(1);
            envelopes.push(encrypt_e2e_payload(
                &dev.device_id,
                secret,
                self.nonce_counter,
                &json,
            ));
        }
        envelopes
    }
}
