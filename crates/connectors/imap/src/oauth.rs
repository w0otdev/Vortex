use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OAuthProvider {
    Gmail,
    Outlook,
}

impl OAuthProvider {
    pub const fn auth_url(self) -> &'static str {
        match self {
            Self::Gmail => "https://accounts.google.com/o/oauth2/v2/auth",
            Self::Outlook => "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
        }
    }

    pub const fn token_host_and_path(self) -> (&'static str, &'static str) {
        match self {
            Self::Gmail => ("oauth2.googleapis.com", "/token"),
            Self::Outlook => ("login.microsoftonline.com", "/common/oauth2/v2.0/token"),
        }
    }

    pub const fn scope(self) -> &'static str {
        match self {
            Self::Gmail => "https://mail.google.com/",
            Self::Outlook => "https://outlook.office.com/IMAP.AccessAsUser.All offline_access",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PkceSession {
    pub provider: OAuthProvider,
    pub client_id: String,
    pub code_verifier: String,
    pub code_challenge: String,
    pub state: String,
    pub redirect_uri: String,
    pub authorize_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokenSet {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
}

pub fn generate_pkce_session(
    provider: OAuthProvider,
    client_id: &str,
    redirect_port: u16,
    seed: u64,
) -> PkceSession {
    let entropy = vortex_core::model::generate_random_bytes_32(b"vortex-oauth-pkce", seed);
    let mut verifier_hasher = Sha256::new();
    verifier_hasher.update(b"vortex-pkce-verifier:");
    verifier_hasher.update(seed.to_le_bytes());
    verifier_hasher.update(entropy);
    verifier_hasher.update(client_id.as_bytes());
    let code_verifier =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(verifier_hasher.finalize());

    let mut challenge_hasher = Sha256::new();
    challenge_hasher.update(code_verifier.as_bytes());
    let code_challenge =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(challenge_hasher.finalize());

    let mut state_hasher = Sha256::new();
    state_hasher.update(b"vortex-oauth-state:");
    state_hasher.update(seed.to_le_bytes());
    state_hasher.update(entropy);
    let state =
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&state_hasher.finalize()[..12]);

    let redirect_uri = format!("http://127.0.0.1:{redirect_port}/oauth/callback");
    let authorize_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&code_challenge={}&code_challenge_method=S256&state={}&access_type=offline&prompt=consent",
        provider.auth_url(),
        url_encode(client_id),
        url_encode(&redirect_uri),
        url_encode(provider.scope()),
        url_encode(&code_challenge),
        url_encode(&state),
    );

    PkceSession {
        provider,
        client_id: client_id.to_string(),
        code_verifier,
        code_challenge,
        state,
        redirect_uri,
        authorize_url,
    }
}

pub fn wait_for_loopback_oauth_code(
    listener: &TcpListener,
    expected_state: &str,
    timeout: Duration,
) -> Result<String, String> {
    let (mut stream, _) = listener
        .accept()
        .map_err(|e| format!("oauth callback accept failed: {e}"))?;
    let _ = stream.set_read_timeout(Some(timeout));

    let mut buf = [0u8; 4096];
    let n = stream
        .read(&mut buf)
        .map_err(|e| format!("oauth callback read failed: {e}"))?;
    let req = String::from_utf8_lossy(&buf[..n]);
    let first_line = req.lines().next().unwrap_or("");
    let path = first_line.split_whitespace().nth(1).unwrap_or("");
    let query = path.split_once('?').map(|(_, q)| q).unwrap_or("");

    let mut code = None;
    let mut state = None;
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            match url_decode(k).as_str() {
                "code" => code = Some(url_decode(v)),
                "state" => state = Some(url_decode(v)),
                _ => {}
            }
        }
    }

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: close\r\n\r\nVortex authentication completed. You may close this tab.";
    let _ = stream.write_all(response.as_bytes());

    if state.as_deref() != Some(expected_state) {
        return Err("oauth state mismatch".into());
    }
    code.ok_or_else(|| "missing authorization code in callback".into())
}

pub fn exchange_code_for_token(
    session: &PkceSession,
    auth_code: &str,
) -> Result<OAuthTokenSet, String> {
    let (host, path) = session.provider.token_host_and_path();
    let form_body = format!(
        "client_id={}&code={}&code_verifier={}&grant_type=authorization_code&redirect_uri={}",
        url_encode(&session.client_id),
        url_encode(auth_code),
        url_encode(&session.code_verifier),
        url_encode(&session.redirect_uri),
    );
    https_post_form_token(host, path, &form_body)
}

pub fn refresh_oauth_token(
    provider: OAuthProvider,
    client_id: &str,
    refresh_token: &str,
) -> Result<OAuthTokenSet, String> {
    let (host, path) = provider.token_host_and_path();
    let form_body = format!(
        "client_id={}&refresh_token={}&grant_type=refresh_token",
        url_encode(client_id),
        url_encode(refresh_token),
    );
    https_post_form_token(host, path, &form_body)
}

fn https_post_form_token(host: &str, path: &str, form_body: &str) -> Result<OAuthTokenSet, String> {
    let tcp =
        TcpStream::connect((host, 443)).map_err(|e| format!("connect {host}:443 failed: {e}"))?;
    let _ = tcp.set_read_timeout(Some(Duration::from_secs(15)));
    let _ = tcp.set_write_timeout(Some(Duration::from_secs(15)));
    let connector = native_tls::TlsConnector::new().map_err(|e| format!("tls init failed: {e}"))?;
    let mut tls = connector
        .connect(host, tcp)
        .map_err(|e| format!("tls handshake failed: {e}"))?;

    let req = format!(
        "POST {path} HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{form_body}",
        form_body.len()
    );
    tls.write_all(req.as_bytes())
        .map_err(|e| format!("token write failed: {e}"))?;
    let mut resp = String::new();
    tls.take(512 * 1024)
        .read_to_string(&mut resp)
        .map_err(|e| format!("token read failed: {e}"))?;
    let (_, body) = resp
        .split_once("\r\n\r\n")
        .ok_or_else(|| "malformed HTTP response".to_string())?;
    let v: serde_json::Value =
        serde_json::from_str(body.trim()).map_err(|e| format!("invalid token JSON: {e}"))?;
    let access_token = v
        .get("access_token")
        .and_then(|x| x.as_str())
        .ok_or_else(|| format!("missing access_token in response: {body}"))?
        .to_string();
    let refresh_token = v
        .get("refresh_token")
        .and_then(|x| x.as_str())
        .map(str::to_string);
    let expires_in = v.get("expires_in").and_then(|x| x.as_u64()).unwrap_or(3600);
    Ok(OAuthTokenSet {
        access_token,
        refresh_token,
        expires_in,
    })
}

pub fn build_xoauth2_sasl(user_email: &str, access_token: &str) -> String {
    let raw = format!("user={user_email}\x01auth=Bearer {access_token}\x01\x01");
    base64::engine::general_purpose::STANDARD.encode(raw.as_bytes())
}

fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            use std::fmt::Write;
            let _ = write!(&mut out, "%{b:02X}");
        }
    }
    out
}

fn url_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'+' {
            out.push(b' ');
            i += 1;
        } else if bytes[i] == b'%'
            && i + 2 < bytes.len()
            && let Ok(hex_str) = std::str::from_utf8(&bytes[i + 1..i + 3])
            && let Ok(val) = u8::from_str_radix(hex_str, 16)
        {
            out.push(val);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}
