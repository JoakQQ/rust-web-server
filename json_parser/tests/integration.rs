use json_parser::decode;

#[test]
fn test_decoding() {
    // let json = r#"{
    //     "name": "John",
    //     "city": "New York",
    //     "age": 30,
    //     "test": {
    //         "xd": 30.69696,
    //         "xdd": {
    //             "xdd": ["12", 12.5435]
    //         }
    //     },
    //     "tests": [
    //         {
    //             "name": "1"
    //         },
    //         {
    //             "name": "1"
    //         }
    //     ]
    // }"#;

    let json = r#"
    {    "test": ["xdd"    ]}
    "#;

    let result = decode::decode_json(&json.to_string()).unwrap();
    println!("result {:#?}", result);
}
