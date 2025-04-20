pub mod database;

use argon2::Argon2;
use sha1::{Digest, Sha1};

pub const SALT: &'_ str = "mI29fmAnxgTs";

//
// This is not the default because it's not cryptographically secure and it's slower than SHA512
//                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//
pub fn gjp2_hash(password: String) -> String {
    let mut hasher = Sha1::new();
    hasher.update(password + SALT);

    let hash = hasher.finalize();
    hash.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn pass_hash(argon2: &Argon2<'_>, password: &String) -> String {
    let mut bytes = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), SALT.as_bytes(), &mut bytes)
        .unwrap();

    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
