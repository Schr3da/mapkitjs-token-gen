#[cfg(test)]
use super::*;

#[test]
pub fn test_generate_with_filepath() {
    let header = TokenHeader::new_with_key_id(&"ASDFGHJKL1", TokenType::JWT);
    let payload = TokenPayload::new_with(&"QWERTZUIOP", 1583533534, 1528476433723, &"https://mywebsite.org");
    let token = generate_with_filepath(&"./key.p8", header, payload);
    
    println!("\nGenerated Token: {}\n", token);
    assert_eq!(true, token.len() > 0)
}

#[test]
pub fn test_generate_with_arguments() {
    let token = generate_with_key_file(
        &"./key.p8",
        &"ASDFGHJKL1",
        TokenType::JWT,
        &"QWERTZUIOP",
        1583533534,
        1528476433723,
        &"https://mywebsite.org"
    );

    println!("\nGenerated Token: {}\n", token);
    assert_eq!(true, token.len() > 0)
}

#[test]
pub fn test_generate_with_data() {
    let data = fs::read("./key.p8".to_string())
        .expect("Provided file key.p8 does not exist");

    let header = TokenHeader::new_with_key_id(&"ASDFGHJKL1", TokenType::JWT);
    let payload = TokenPayload::new_with(&"QWERTZUIOP", 1583533534, 1528476433723, &"https://mywebsite.org");
    let token = generate_with_data(data, header, payload);
    
    println!("\nGenerated Token: {}\n", token);
    assert_eq!(true, token.len() > 0)
}

#[test]
pub fn test_generate_with_data_and_other_arguments() {
    let data = fs::read("./key.p8".to_string())
        .expect("Provided file key.p8 does not exist");

    let token = generate_with_key_data(
        data,
        &"ASDFGHJKL1",
        TokenType::JWT,
        &"QWERTZUIOP",
        1583533534,
        1528476433723,
        &"https://mywebsite.org" 
    );

    println!("\nGenerated Token: {}\n", token);
    assert_eq!(true, token.len() > 0)
}
