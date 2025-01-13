use argon2::{password_hash::{
    rand_core::OsRng,
    PasswordHash, PasswordHasher, PasswordVerifier, SaltString
}, Argon2};

/// Hashes a password using the Argon2 algorithm and updates the original password with the hash.
///
/// # Parameters
/// - `password`: A mutable reference to a `String` containing the password to be hashed.
///   The original password will be replaced with its hashed version.
///
/// # Returns
/// A `Result` containing the hashed password as a `String` if successful, or an `argon2::password_hash::Error`
/// if the hashing process fails.
pub fn hash_password(password: &mut String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    *password = hash.to_string();
    Ok(hash.to_string())
}

pub fn hash_password_ret(password: &String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    let password = hash.to_string();
    Ok(password)
}

/// Verifies a password against a hashed password using the Argon2 algorithm.
///
/// # Parameters
/// - `password`: A string slice representing the plain text password to verify.
/// - `hashed_password`: A string slice containing the hashed password to verify against.
///
/// # Returns
/// A `Result` which is `Ok(())` if the password matches the hashed password, or an `Err(String)`
/// containing an error message if the verification fails.
pub fn verify_password(password: &str, hashed_password: &str) -> Result<(), String> {
    let parsed_hash = PasswordHash::new(hashed_password).map_err(|err| {
        format!("Failed to parse hashed password: {}", err)
    })?;

    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).map_err(|err| {
        format!("Password verification failed: {}", err)
    })
}