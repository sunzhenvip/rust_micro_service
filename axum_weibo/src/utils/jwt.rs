use serde::{Deserialize, Serialize};
use anyhow::Result;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, TokenData, Validation};
use crate::config::app_config::APP_CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub uid: u32,
    pub exp: usize,
    pub iat: usize
}

pub fn create_jwt(my_claims: Claims) -> Result<String> {
    let jwt = &APP_CONFIG.get().unwrap().jwt;
    let token = encode(&Header::default(),
                       &my_claims,
                       &EncodingKey::from_secret(jwt.secret.as_ref()))?;
    Ok(token)
}

pub fn check_token(token: &str) -> Result<TokenData<Claims>> {
    let jwt = &APP_CONFIG.get().unwrap().jwt;
    let c = decode::<Claims>(&token,
                             &DecodingKey::from_secret(jwt.secret.as_str().as_ref()),
                             &Validation::default())?;
    Ok(c)
}