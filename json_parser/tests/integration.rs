use json_parser::{decode, encode, JsonNode};
use std::collections::HashMap;

#[test]
fn test_decoding() {
    let json = r#"{
        "name": "John",
        "city": "New York",
        "age": 30,
        "phones": [
            "12345678",
            "12345679"
        ],
        "friends": [
            {
                "name": "Mary"
            },
            {
                "name": "Steven"
            }
        ]
    }"#;
    let result = decode::decode_json(&json.to_string()).unwrap();

    let mut obj: HashMap<String, JsonNode> = HashMap::new();
    obj.insert("name".to_string(), JsonNode::String("John".to_string()));
    obj.insert("city".to_string(), JsonNode::String("New York".to_string()));
    obj.insert("age".to_string(), JsonNode::Number(30.0));
    let mut friend1: HashMap<String, JsonNode> = HashMap::new();
    friend1.insert("name".to_string(), JsonNode::String("Mary".to_string()));
    let mut friend2: HashMap<String, JsonNode> = HashMap::new();
    friend2.insert("name".to_string(), JsonNode::String("Steven".to_string()));
    let friend_list = vec![JsonNode::Object(friend1), JsonNode::Object(friend2)];
    let phone_list: Vec<JsonNode> = vec![
        JsonNode::String("12345678".to_string()),
        JsonNode::String("12345679".to_string()),
    ];
    obj.insert("phones".to_string(), JsonNode::Array(phone_list));
    obj.insert("friends".to_string(), JsonNode::Array(friend_list));
    let node = JsonNode::Object(obj);

    // println!("result {:#?}", result);
    assert!(node == result);
}

#[test]
fn test_encoding() {
    let mut obj: HashMap<String, JsonNode> = HashMap::new();
    obj.insert("name".to_string(), JsonNode::String("John".to_string()));
    obj.insert("city".to_string(), JsonNode::String("New York".to_string()));
    obj.insert("age".to_string(), JsonNode::Number(30.0));

    let mut friend1: HashMap<String, JsonNode> = HashMap::new();
    friend1.insert("name".to_string(), JsonNode::String("Mary".to_string()));
    let mut friend2: HashMap<String, JsonNode> = HashMap::new();
    friend2.insert("name".to_string(), JsonNode::String("Steven".to_string()));
    let friend_list = vec![JsonNode::Object(friend1), JsonNode::Object(friend2)];
    let phone_list: Vec<JsonNode> = vec![
        JsonNode::String("12345678".to_string()),
        JsonNode::String("12345679".to_string()),
    ];
    obj.insert("phones".to_string(), JsonNode::Array(phone_list));
    obj.insert("friends".to_string(), JsonNode::Array(friend_list));
    let node = JsonNode::Object(obj);

    let s = encode::encode_json(&node);
    println!("s {}", s);
    assert!(s.len() > 0);
}
