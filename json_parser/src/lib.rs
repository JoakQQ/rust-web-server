use std::collections::HashMap;

// start = false, end = true
#[derive(PartialEq, Debug)]
enum StackItem {
    Key(bool),
    Value(bool),
    Object(bool),
    Array(bool),
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum JsonNode {
    Number(f64),
    String(String),
    Object(HashMap<String, JsonNode>),
    Array(Vec<JsonNode>),
}

pub mod decode;
pub mod encode;
