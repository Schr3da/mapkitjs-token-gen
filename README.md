# mapkitjs-token-gen
This utility library provides wrapper functions to generate MapKitJS valid jwt tokens for your rust program. 

You can find more details about MapKitJS using the offical [Apple Dokumentation](https://developer.apple.com/documentation/mapkitjs)

## Usage

### Example 1
```
let header = TokenHeader::new_with_key_id(&"ASDFGHJKL1", TokenType::JWT);
let payload = TokenPayload::new_with(&"QWERTZUIOP", 1583533534, 1528476433723, &"http://my-website.com");
let token = generate_with_filepath(&"./key.p8", header, payload);
println("{}", token);
```

### Example 2
```
let token = generate_with_key_file(
  &"./key.p8",
  &"ASDFGHJKL1",
  TokenType::JWT,
  &"QWERTZUIOP",
  1583533534,
  1528476433723,
  &"http://my-website.com"
);
println("{}, token");
```

### Example 3
```
let token = generate_with_key_data(
    data,
    &"ASDFGHJKL1",
    TokenType::JWT,
    &"QWERTZUIOP",
    1583533534,
    1528476433723,
    &"http://my-website.com"
);
println("{}", token);
```

### Example 4
```
let data = fs::read(&"./key.p8")
    .expect("Provided file key.p8 does not exist");

let header = TokenHeader::new_with_key_id(&"ASDFGHJKL1", TokenType::JWT);
let payload = TokenPayload::new_with(&"QWERTZUIOP", 1583533534, 1528476433723, &"http://my-websize.com");
let token = generate_with_data(data, header, payload);
println("{}", token);
```

