use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, Result, SaltString,
    },
    Algorithm, Argon2, Params, Version,
};
pub fn hash_password(secret: &[u8], password: &[u8]) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::new_with_secret(
        secret,
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )?;

    // Hash password to PHC string ($argon2id$v=19$...)

    Ok(argon2.hash_password(password, &salt)?.to_string())
}

pub fn verify_password(secret: &[u8], password: &[u8], password_hash: &str) -> Result<bool> {
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::new_with_secret(
        secret,
        Algorithm::default(),
        Version::default(),
        Params::default(),
    )?;

    // Verify password against PHC string.
    //
    // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // `Argon2` instance.
    let parsed_hash = PasswordHash::new(password_hash)?;

    Ok(argon2.verify_password(password, &parsed_hash).is_ok())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash_and_verify_correct_password() {
        let password = "it_should_work";
        let secret = "secret";

        let hashed_password = hash_password(secret.as_bytes(), password.as_bytes()).unwrap();

        assert!(verify_password(
            secret.as_bytes(),
            password.as_bytes(),
            hashed_password.as_str()
        )
        .unwrap())
    }

    #[test]
    fn hash_and_verify_wrong_password() {
        let password = "it_should_not_work";
        let secret = "secret";

        let hashed_password = hash_password(password.as_bytes(), secret.as_bytes()).unwrap();

        assert!(!verify_password(
            secret.as_bytes(),
            "wrong_password".as_bytes(),
            hashed_password.as_str()
        )
        .unwrap())
    }
}

