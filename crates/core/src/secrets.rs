use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const SERVICE_NAME: &str = "vortex-inbox";

#[derive(Clone)]
pub enum SecretStore {
    OsKeyring(Arc<Mutex<HashMap<String, String>>>),
    Memory(Arc<Mutex<HashMap<String, String>>>),
}

impl Default for SecretStore {
    fn default() -> Self {
        Self::new_os()
    }
}

impl SecretStore {
    pub fn new_os() -> Self {
        Self::OsKeyring(Arc::new(Mutex::new(HashMap::new())))
    }

    pub fn new_memory() -> Self {
        Self::Memory(Arc::new(Mutex::new(HashMap::new())))
    }

    pub fn set_secret(&self, key: &str, secret: &str) -> Result<(), String> {
        match self {
            Self::OsKeyring(cache) => {
                if let Ok(mut guard) = cache.lock() {
                    guard.insert(key.to_string(), secret.to_string());
                }
                let entry = keyring::Entry::new(SERVICE_NAME, key)
                    .map_err(|e| format!("keyring init error: {e}"))?;
                entry
                    .set_password(secret)
                    .map_err(|e| format!("keyring store error: {e}"))
            }
            Self::Memory(map) => {
                map.lock()
                    .map_err(|_| "memory secret lock poisoned".to_string())?
                    .insert(key.to_string(), secret.to_string());
                Ok(())
            }
        }
    }

    pub fn get_secret(&self, key: &str) -> Result<Option<String>, String> {
        match self {
            Self::OsKeyring(cache) => {
                if let Ok(guard) = cache.lock()
                    && let Some(val) = guard.get(key)
                {
                    return Ok(Some(val.clone()));
                }
                let entry = keyring::Entry::new(SERVICE_NAME, key)
                    .map_err(|e| format!("keyring init error: {e}"))?;
                match entry.get_password() {
                    Ok(pw) => {
                        if let Ok(mut guard) = cache.lock() {
                            guard.insert(key.to_string(), pw.clone());
                        }
                        Ok(Some(pw))
                    }
                    Err(keyring::Error::NoEntry) => Ok(None),
                    Err(e) => Err(format!("keyring read error: {e}")),
                }
            }
            Self::Memory(map) => Ok(map
                .lock()
                .map_err(|_| "memory secret lock poisoned".to_string())?
                .get(key)
                .cloned()),
        }
    }

    pub fn delete_secret(&self, key: &str) -> Result<(), String> {
        match self {
            Self::OsKeyring(cache) => {
                if let Ok(mut guard) = cache.lock() {
                    guard.remove(key);
                }
                let entry = keyring::Entry::new(SERVICE_NAME, key)
                    .map_err(|e| format!("keyring init error: {e}"))?;
                match entry.delete_credential() {
                    Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
                    Err(e) => Err(format!("keyring delete error: {e}")),
                }
            }
            Self::Memory(map) => {
                map.lock()
                    .map_err(|_| "memory secret lock poisoned".to_string())?
                    .remove(key);
                Ok(())
            }
        }
    }
}
