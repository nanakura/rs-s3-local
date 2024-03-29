use crypto_hash::{hex_digest, Algorithm};

// const DEFAULT_KEY: [u8; 8] = [76, 111, 99, 97, 108, 83, 51, 88];
#[allow(dead_code)]
static DEFAULT_KEY: &str = "LocalS3X";

pub fn encrypt_by_md5(s: &str) -> String {
    let digest = hex_digest(Algorithm::MD5, s.as_bytes());
    digest
}

#[allow(dead_code)]
pub fn encrypt_by_des(data: &str) -> anyhow::Result<String> {
    Ok(data.to_string())
}

#[allow(dead_code)]
pub fn decrypt_by_des(data: &str) -> anyhow::Result<String> {
    Ok(data.to_string())
}