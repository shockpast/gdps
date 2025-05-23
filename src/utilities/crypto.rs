use std::sync::Arc;

use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE},
};
use sha1::Digest as sha1Digest;

pub fn singluar_xor(string: &[u8], key: &[u8]) -> String {
    let mut result: String = String::new();

    for i in 0..string.len() {
        let c = string[i] ^ key[i % key.len()];
        result.push(c as char);
    }

    result
}

pub fn cyclic_xor(input: &str, key: &str) -> String {
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();

    input
        .bytes()
        .enumerate()
        .map(|(i, b)| {
            let k = key_bytes[i % key_len];
            (b ^ k) as char
        })
        .collect()
}

pub fn sha1_salt(base: &String, salt: &str) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(base);
    hasher.update(salt);

    hex::encode(hasher.finalize().as_slice())
}

pub fn hash_level_string(level_string: &String) -> String {
    let mut lstring: String = String::new();

    for (counter, i) in (0_i16..).zip((0..level_string.len()).step_by(level_string.len() / 40)) {
        if counter == 40 {
            break;
        }

        lstring.push(level_string.as_bytes()[i] as char);
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
    hex::encode(STANDARD.decode(input).unwrap_or_default())
}

pub fn decode_base64_url(input: &str) -> String {
    hex::encode(
        STANDARD
            .decode(input.replace('-', "+").replace('_', "/"))
            .unwrap_or_default(),
    )
}

pub fn decode_base64_url_raw(input: &str) -> Vec<u8> {
    STANDARD
        .decode(input.replace('-', "+").replace('_', "/"))
        .unwrap_or_default()
}

pub fn generate_checksum(mut values: Vec<String>, key: &str, salt: &str) -> String {
    values.push(salt.to_string());

    let value_str = values.join("");

    let mut hasher = sha1::Sha1::new();
    hasher.update(value_str);

    let hashed = hasher.finalize();
    let hashed: &[u8] = hashed.as_slice();
    let xored: String = singluar_xor(hex::encode(hashed).as_bytes(), key.as_bytes()).to_string();

    encode_base64(&xored)
}
