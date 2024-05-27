use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub url: String,
    pub qr_str: String,
}

impl Auth {
    pub fn new(url: String, qr_str: String) -> Self {
        Self { url, qr_str }
    }
}
