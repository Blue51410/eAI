use crate::data::base_dir;
use pbkdf2::pbkdf2_hmac;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::path::PathBuf;
use uuid::Uuid;

/// PBKDF2 iterations make a stolen accounts file slow to brute-force.
const ITERATIONS: u32 = 100_000;
const KEY_LEN: usize = 32;

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: Uuid,
    pub name: String,
    /// Salt for the password hash. Empty when the profile has no password.
    #[serde(default)]
    pub salt: String,
    /// Hex-encoded password hash. `None` means the profile has no password.
    #[serde(default)]
    pub password: Option<String>,
}

impl Account {
    pub fn has_password(&self) -> bool {
        self.password.is_some()
    }

    pub fn verify(&self, password: &str) -> bool {
        let Some(expected) = self.password.as_deref() else {
            return true;
        };
        hash_password(password, &self.salt) == expected
    }
}

fn accounts_file() -> PathBuf {
    base_dir().join("accounts.json")
}

pub fn load_accounts() -> Vec<Account> {
    std::fs::read_to_string(accounts_file())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_accounts(accounts: &[Account]) {
    let _ = std::fs::create_dir_all(base_dir());
    if let Ok(json) = serde_json::to_string_pretty(accounts) {
        let path = accounts_file();
        let _ = std::fs::write(&path, json);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        }
    }
}

pub fn new_salt() -> String {
    Uuid::new_v4().simple().to_string()
}

/// PBKDF2-HMAC-SHA256 key derivation. A per-profile random salt is combined
/// with the password inside a standard KDF, so a stolen accounts file can't
/// be brute-forced quickly.
pub fn hash_password(password: &str, salt: &str) -> String {
    let mut out = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt.as_bytes(), ITERATIONS, &mut out);
    to_hex(&out)
}

fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write;
        let _ = write!(&mut out, "{byte:02x}");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_round_trip() {
        let salt = new_salt();
        let password = format!("pw-{}", Uuid::new_v4());
        let hash = hash_password(&password, &salt);
        assert_ne!(hash, password);
        assert_eq!(hash.len(), 64);
        assert_eq!(hash_password(&password, &salt), hash);
    }

    #[test]
    fn account_checks_password() {
        let salt = new_salt();
        let password = format!("pw-{}", Uuid::new_v4());
        let account = Account {
            id: Uuid::new_v4(),
            name: Uuid::new_v4().simple().to_string(),
            salt: salt.clone(),
            password: Some(hash_password(&password, &salt)),
        };
        assert!(account.verify(&password));
        assert!(!account.verify(&format!("{password}-x")));
        assert!(account.has_password());
    }

    #[test]
    fn account_without_password_accepts_anything() {
        let account = Account {
            id: Uuid::new_v4(),
            name: Uuid::new_v4().simple().to_string(),
            salt: String::new(),
            password: None,
        };
        assert!(account.verify(""));
        assert!(account.verify(&Uuid::new_v4().to_string()));
        assert!(!account.has_password());
    }
}
