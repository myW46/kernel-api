use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::error::AppError;


pub fn hash(password: &str) -> Result<String, AppError> {
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


#[cfg(test)]
    mod tests{
        use super::*;
        #[test]
        fn hash_and_verify_password(){
            let pwd_example="password_example";
            let hashed_example=hash(pwd_example).expect("cannot hashing example");
            assert!(verify(pwd_example, &hashed_example));
            assert!(!verify("wrong", &hashed_example));
        }
        #[test]
        fn empty_password(){
            let pwd_example="password_example";
            let hash1=hash(pwd_example).expect("cannot hashing same");
            let hash2 =hash(pwd_example).expect("cannot hashing same");
            assert_ne!(hash1, hash2);
        }


    }
