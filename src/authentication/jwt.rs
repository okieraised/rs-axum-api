use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, Error};
use crate::constants::jwt_constants::JWT_SECRET;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    aud: Vec<String>,
    role: String,
    exp: u64,
    nbf: Option<u64>,
    iat: u64,
    jti: uuid::Uuid,
}

pub fn new_jwt(subject: &str, audience: &str, role: &str) -> Result<String> {

    let current_time_result = SystemTime::now().duration_since(UNIX_EPOCH);
    let current_time = match current_time_result {
        Ok(t) => t,
        Err(err) => {
            return Err(Error::new(err));
        }
    };

    let mut aud: Vec<String> = Vec::new();
    aud.push(String::from(audience));

    let claim = Claims {
        sub: subject.to_owned(),
        aud: aud,
        role: role.to_owned(),
        exp: current_time.as_secs() + 3600,
        nbf: Option::from(current_time.as_secs()),
        iat: current_time.as_secs(),
        jti: uuid::Uuid::new_v4(),
    };
    let header = Header::new(Algorithm::HS512);
    return match encode(&header, &claim, &EncodingKey::from_secret(JWT_SECRET)) {
        Ok(token) => {
            Ok(token)
        }
        Err(err) => {
            Err(Error::new(err))
        }
    };
}

pub fn decode_jwt(jwt: &str) -> Result<Claims> {

    let mut validation = Validation::new(Algorithm::HS512);

    let token_data = decode::<Claims>(jwt, &DecodingKey::from_secret(JWT_SECRET), &Validation::new(Algorithm::HS512));
    let claim = match token_data {
        Ok(claims) => {
            Ok(claims.claims)
        }
        Err(err) => {
            Err(Error::new(err))
        }
    };
    claim
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_curent_time() {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => println!("curent time: {}", n.as_secs()),
            Err(_) => panic!("panic"),
        }
    }

    #[test]
    fn test_new_jwt_token() {
        let token_result = match new_jwt("tripg", "test_api", "admin") {
            Ok(token) => {
                println!("{}", token);
            }
            Err(_) => {

            }
        };
    }

    #[test]
    fn test_decode_jwt_token() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJ0cmlwZyIsImF1ZCI6WyJ0ZXN0X2FwaSJdLCJyb2xlIjoiYWRtaW4iLCJleHAiOjE3MDA0ODA1MTEsIm5iZiI6MTcwMDQ3NjkxMSwiaWF0IjoxNzAwNDc2OTExLCJqdGkiOiIxOTc3MTYzYi02NGQ3LTQ3YzItODczNS04ZWM1MTViYTkzNjcifQ.heRfWgxGBI7wXIWKPM4I2DN4b5_d03q0fOKUXiXAyjRggVVF670kslMW1ufGobOL3214IlEhrY_VuvXL7NR_Aw";
        let claims = match decode_jwt(&token) {
            Ok(cl) => {
                println!("{:#?}", cl)
            }
            Err(err) => {
                println!("error occurred: {}", err.to_string())
            }
        };

    }

}