use argon2::{
	password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
	Argon2,
};

use crate::error::AppError;

pub fn hash(password: &str)-> Result<String,AppError>{
	let salt = SaltString::generate(&mut OsRng);
	let argon2 = Argon2::default();

	let password_hash = argon2
		.hash_password(password.as_bytes(), &salt)
		.map_err(|_| AppError::InternalError)?
		.to_string();
	Ok(password_hash)
}

pub fn verify(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}