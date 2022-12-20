use serde::{Serialize, Deserialize};
use sha::{sha256::Sha256, utils::{Digest, DigestExt}};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    password: String
}

impl Config {
    pub fn hash_passwd(&mut self) {
        self.password = Sha256::default().digest(self.password.as_bytes()).to_hex();
    }
}