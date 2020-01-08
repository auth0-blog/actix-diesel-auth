use crate::errors::ServiceError;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let domain = std::env::var("DOMAIN").expect("DOMAIN must be set");
    let jwks = fetch_jwks(&format!("{}{}", domain.as_str(), ".well-known/jwks.json"))
        .expect("failed to fetch jwks");
    let validations = vec![Validation::Issuer(domain), Validation::SubjectPresent];
    let kid = match token_kid(&token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => return Err(ServiceError::JWKSFetchError),
    };
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let res = validate(token, jwk, validations);
    Ok(res.is_ok())
}

fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let mut res = reqwest::get(uri)?;
    let val = res.json::<JWKS>()?;
    return Ok(val);
}
