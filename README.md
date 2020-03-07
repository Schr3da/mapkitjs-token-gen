# mapkitjs-token-gen
This is an utility library to generate jwt token for mapkitjs. The jwt token is required in order to use the mapkit. 
This library provides helper function to create quickly valid tokens.

You can find more details about mapkitjs reading the offical docs
[Offical Apple MapKitjs Documentation](https://developer.apple.com/documentation/mapkitjs)

## Usage

### Example 1
```
let header = TokenHeader::new_with_key_id("ASDFGHJKL1".to_string(), TokenType::JWT);
let payload = TokenPayload::new_with("QWERTZUIOP".to_string(), 1583533534, 1528476433723);
let token = generate_with_filepath("./key.p8".to_string(), header, payload);
println("{}", token);
```

### Example 2
```
let token = generate_with_key_file(
  "./key.p8".to_string(),
  "ASDFGHJKL1".to_string(),
  TokenType::JWT,
  "QWERTZUIOP".to_string(),
  1583533534,
  1528476433723
);
println("{}, token");
```

### Example 3
```
let token = generate_with_key_data(
    data,
    "ASDFGHJKL1".to_string(),
    TokenType::JWT,
    "QWERTZUIOP".to_string(),
    1583533534,
    1528476433723
);
```

### Example 4
```
let data = fs::read("./key.p8".to_string())
    .expect("Provided file key.p8 does not exist");

let header = TokenHeader::new_with_key_id("ASDFGHJKL1".to_string(), TokenType::JWT);
let payload = TokenPayload::new_with("QWERTZUIOP".to_string(), 1583533534, 1528476433723);
let token = generate_with_data(data, header, payload);
```

