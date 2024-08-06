use argon2::{
    password_hash::SaltString, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    Version,
};
use secrecy::{ExposeSecret as _, Secret};

use crate::{Error, Result};

/// Hashes a plain text password and returns the hashed result.
///
/// # Errors
///
/// Return [`argon2::password_hash::Result`] when could not hash the given
/// password.
///
/// # Example
/// ```rust
/// use loco_rs::hash;
///
/// hash::hash_password("password-to-hash");
/// ```
pub fn hash_password(pass: &Secret<String>) -> Result<String> {
    let arg2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::default(),
    );
    let salt = SaltString::generate(&mut rand::rngs::ThreadRng::default());

    Ok(arg2
        .hash_password(pass.expose_secret().as_bytes(), &salt)
        .map_err(|err| Error::Hash(err.to_string()))?
        .to_string())
}

/// Verifies a plain text password against a hashed password.
///
/// # Errors
///
/// Return [`argon2::password_hash::Result`] when could verify the given data.
///
/// # Example
/// ```rust
/// use loco_rs::hash;
///
/// hash::verify_password("password", "hashed-password");
/// ```
#[must_use]
pub fn verify_password(pass: &Secret<String>, hashed_password: &str) -> bool {
    let arg2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    );
    let Ok(hash) = PasswordHash::new(hashed_password) else {
        return false;
    };
    arg2.verify_password(pass.expose_secret().as_bytes(), &hash)
        .is_ok()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_hash_password() {
        let pass = Secret::new("password-1234".to_owned());

        let hash_pass = hash_password(&pass).unwrap();

        assert!(verify_password(&pass, &hash_pass));
    }
}
