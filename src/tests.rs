#[cfg(test)]
use super::*;

#[test]
pub fn test_generate_with_filepath() {
    let header = TokenHeader::new_with_key_id("ASDFGHJKL1".to_string(), TokenType::JWT);
    let payload = TokenPayload::new_with("QWERTZUIOP".to_string(), 1583533534, 1528476433723);
    let token = generate_with_filepath("./key.p8".to_string(), header, payload);
    
    println!("\nGenerated Token: {}\n", token);
    assert_eq!(true, token.len() > 0)
}

#[test]
pub fn test_generate_with_arguments() {
    let token = generate_with_key_file(
        "./key.p8".to_string(),
        "ASDFGHJKL1".to_string(),
        TokenType::JWT,
        "QWERTZUIOP".to_string(),
        1583533534,
        1528476433723
    );

    println!("\nGenerated Token: {}\n", token);
    assert_eq!(true, token.len() > 0)
}

#[test]
pub fn test_generate_with_data() {
    let data = fs::read("./key.p8".to_string())
        .expect("Provided file key.p8 does not exist");

    let header = TokenHeader::new_with_key_id("ASDFGHJKL1".to_string(), TokenType::JWT);
    let payload = TokenPayload::new_with("QWERTZUIOP".to_string(), 1583533534, 1528476433723);
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
        "ASDFGHJKL1".to_string(),
        TokenType::JWT,
        "QWERTZUIOP".to_string(),
        1583533534,
        1528476433723
    );

    println!("\nGenerated Token: {}\n", token);
    assert_eq!(true, token.len() > 0)
}
