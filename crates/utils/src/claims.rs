use crate::settings::structs::Settings;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

type Jwt = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// user_id, standard claim by RFC 7519.
    pub user_name: String,
    pub password: String,
    /// Time when this token was issued as UNIX-timestamp in seconds
    pub iat: i64,
}

impl Claims {
    pub fn decode(jwt: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let v = Validation {
            validate_exp: false,
            ..Validation::default()
        };
        decode::<Claims>(
            &jwt,
            &DecodingKey::from_secret(Settings::get().jwt_secret().as_ref()),
            &v,
        )
    }

    pub fn jwt(user_name: String, password: String) -> Result<Jwt, jsonwebtoken::errors::Error> {
        let claims = Claims {
            user_name: user_name,
            password: password,
            iat: Utc::now().timestamp(),
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(Settings::get().jwt_secret().as_ref()),
        )
    }
}
