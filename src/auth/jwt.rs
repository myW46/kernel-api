use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use crate::error::AppError;

#[derive(Deserialize, Serialize)]
pub struct Claims {
	pub sub:uuid::Uuid,
	pub exp:i64,
}

pub fn sign(user_id: uuid::Uuid) -> Result<String,AppError>{
	let exp = (Utc::now()+Duration::hours(24)).timestamp();
	let secret="secret key";

	let claim = Claims{
		sub:user_id, 
		exp,
	};

	encode(
		&Header::default(), 
		&claim, 
		&EncodingKey::from_secret(secret.as_ref()),
		)
		.map_err(|_| AppError::InternalError)
} 