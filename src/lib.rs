extern crate jsonwebtoken;
extern crate serde;

mod tests;

use jsonwebtoken::{encode, Algorithm, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenType {
   JWT 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenHeader {
    kid: Option<String>,
    typ: TokenType,
}

impl TokenHeader {
    pub fn new(kid: &str, typ: TokenType) -> Self {
        TokenHeader { 
            kid: Option::Some(kid.to_string()), 
            typ,
        }
    }

    pub fn new_with_key_id(key_id: &str, typ: TokenType) -> Self {
        TokenHeader::new(key_id, typ)
    }

    pub fn typ_to_value(&self) -> Option<String>{
        let value = match self.typ {
            TokenType::JWT => "JWT".to_string(),
        };
        Option::Some(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload{
    iss: String,
    iat: i64,
    exp: i64,
    origin: String,
}

impl TokenPayload{
    pub fn new(
        iss: &str,
        iat: i64,
        exp: i64,
        origin: &str 
    ) -> Self {
        TokenPayload { 
            iss: iss.to_string(),
            iat,
            exp,
            origin: origin.to_string(),
        }
    }

    pub fn new_with(
        team_id: &str, 
        created_at: i64, 
        expires_at: i64,
        origin: &str
    ) -> Self {
        TokenPayload::new(team_id, created_at, expires_at, origin)
    }
}

pub fn generate_with_filepath(
    filepath: &str,
    header: TokenHeader,
    payload: TokenPayload
) -> String {
    let data = fs::read(filepath)
        .expect("Unable to read file");
    
    generate_with_data(data, header, payload)
}

pub fn generate_with_key_file(
    key_file: &str,
    key_id: &str,
    typ: TokenType,
    team_id: &str,
    created_at: i64,
    expires_at: i64,
    origin: &str,
) -> String {
    let header = TokenHeader::new_with_key_id(key_id, typ);
    let payload = TokenPayload::new_with(team_id, created_at, expires_at, origin);
    generate_with_filepath(key_file, header, payload)
}

pub fn generate_with_key_data(
    key_data: Vec<u8>,
    key_id: &str,
    typ: TokenType,
    team_id: &str,
    created_at: i64,
    expires_at: i64,
    origin: &str, 
) -> String {
    let header = TokenHeader::new_with_key_id(key_id, typ);
    let payload = TokenPayload::new_with(team_id, created_at, expires_at, origin);
    generate_with_data(key_data, header, payload)
}

pub fn generate_with_data(
    data: Vec<u8>,
    header: TokenHeader,
    payload: TokenPayload
) -> String {
   let encoded_key = match  EncodingKey::from_ec_pem(&data) {
        Ok(v) => v, 
        Err(e) => panic!(e), 
    };

    let mut _header = Header::new(Algorithm::ES256);
    _header.kid = header.kid.clone(); 
    _header.typ = header.typ_to_value();

    encode(&_header, &payload, &encoded_key).unwrap()
}

