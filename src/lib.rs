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
    pub fn new(kid: String, typ: TokenType) -> Self {
        TokenHeader { 
            kid: Option::Some(kid), 
            typ,
        }
    }

    pub fn new_with_key_id(key_id: String, typ: TokenType) -> Self {
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
}

impl TokenPayload{
    pub fn new(iss: String, iat: i64, exp: i64) -> Self {
        TokenPayload { iss, iat, exp}
    }

    pub fn new_with(team_id: String, created_at: i64, expires_at: i64) -> Self {
        TokenPayload::new(team_id, created_at, expires_at)
    }
}

pub fn generate_with_filepath(
    filepath: String,
    header: TokenHeader,
    payload: TokenPayload
) -> String {
    let data = fs::read(filepath)
        .expect("Unable to read file");
    
    generate_with_data(data, header, payload)
}

pub fn generate_with_key_file(
    key_file: String,
    key_id: String,
    typ: TokenType,
    team_id: String,
    created_at: i64,
    expires_at: i64,
) -> String {
    let header = TokenHeader::new_with_key_id(key_id, typ);
    let payload = TokenPayload::new_with(team_id, created_at, expires_at);
    generate_with_filepath(key_file, header, payload)
}

pub fn generate_with_key_data(
    key_data: Vec<u8>,
    key_id: String,
    typ: TokenType,
    team_id: String,
    created_at: i64,
    expires_at: i64,
) -> String {
    let header = TokenHeader::new_with_key_id(key_id, typ);
    let payload = TokenPayload::new_with(team_id, created_at, expires_at);
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




