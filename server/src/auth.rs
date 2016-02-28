
use crypto::scrypt::{scrypt_simple, ScryptParams, scrypt_check};
use std::io;
use std::result::Result;

pub fn hash_password(password_unhashed: String) -> io::Result<String> {
    let scrypt_params = ScryptParams::new(10, 8, 1);
    scrypt_simple(&password_unhashed, &scrypt_params)
}

pub fn verify_password(password_hashed: String, to_check: String) -> Result<bool, &'static str> {
        scrypt_check(&password_hashed, &to_check)
}
