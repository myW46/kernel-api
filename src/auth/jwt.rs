use crate::error::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, encode, decode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub exp: i64,
}

pub fn sign(user_id: uuid::Uuid) -> Result<String, AppError> {
    let exp = (Utc::now() + Duration::hours(24)).timestamp();
    let secret = "secret key";

    let claim = Claims { sub: user_id, exp };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| AppError::InternalError)
}

pub fn verify(token: &str)->Result<uuid::Uuid, AppError>{
    let secret = "secret key";
    let decoding_key=DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::default();
    let token_data=decode::<Claims>(
        token,
        &decoding_key,
        &validation,
    ).map_err(|_| AppError::Unauthorized)?;

    Ok(token_data.claims.sub)
}

#[cfg(test)]
mod tests{
    use super::*;
    use uuid::Uuid;
    #[test]
    fn validate(){
        let user_id = Uuid::new_v4();
        let test_token=sign(user_id).expect("Failet to sind");
        let decode_id=verify(&test_token).expect("Failed to decode");
        assert_eq!(user_id, decode_id)

    }

}
