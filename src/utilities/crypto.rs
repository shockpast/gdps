use std::sync::Arc;

use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE},
};
use sha1::Digest as sha1Digest;
use sha2::Digest;

pub fn hash(message: String, salt: String) -> String {
    let mut sha256 = sha2::Sha256::new();

    sha256.update(salt.as_bytes());
    sha256.update(message.as_bytes());

    hex::encode_upper(sha256.finalize())
}

pub fn xor_cipher(string: &[u8], key: &[u8]) -> String {
    let mut result: String = String::new();

    for i in 0..string.len() {
        let c = string[i] ^ key[i % key.len()];
        result.push(c as char);
    }

    result
}

pub fn sha1_salt(base: &String, salt: &str) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(base);
    hasher.update(salt);

    hex::encode(hasher.finalize().as_slice())
}

pub fn hash_level_string(level_string: String) -> String {
    let mut lstring: String = String::new();
    let mut counter: i16 = 0;

    for k in (0..level_string.len()).step_by(level_string.len() / 40) {
        if counter == 40 {
            break;
        }
        lstring.push(level_string.as_bytes()[k] as char);
        counter += 1;
    }

    sha1_salt(&lstring, "xI25fpAapCQg")
}

pub async fn hash_password(password: &str) -> String {
    let password = password.to_string();
    let salt = Arc::new(SaltString::from_b64("mI29fmAnxgTs").unwrap());

    tokio::task::spawn_blocking(move || {
        let salt = Arc::clone(&salt);

        Argon2::default()
            .hash_password(password.as_bytes(), &*salt)
            .unwrap()
            .to_string()
    })
    .await
    .unwrap()
}

pub fn encode_base64(input: &str) -> String {
    STANDARD.encode(input)
}

pub fn encode_base64_url(input: &str) -> String {
    URL_SAFE.encode(input)
}

pub fn decode_base64(input: &str) -> String {
    hex::encode_upper(STANDARD.decode(input).unwrap_or_default())
}

pub fn decode_base64_url(input: &str) -> String {
    hex::encode_upper(URL_SAFE.decode(input).unwrap_or_default())
}
