use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use error_stack::ResultExt;

use crate::errors::AppError;
use crate::errors::AppResult;

pub fn hash(plain_password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2::Argon2::default();
    let hashed_password = argon2
        .hash_password(plain_password.as_bytes(), &salt)
        .change_context(AppError)?;

    Ok(hashed_password.to_string())
}

pub fn verify(input_password: &str, hashed_password: &str) -> AppResult<bool> {
    let argon2 = argon2::Argon2::default();
    let hashed_password = PasswordHash::new(hashed_password).change_context(AppError)?;
    let verified = argon2.verify_password(input_password.as_bytes(), &hashed_password);

    Ok(verified.is_ok())
}
