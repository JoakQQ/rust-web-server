use std::collections::{VecDeque, HashMap};

use crate::{JsonNode, StackItem};

pub type DecodeJsonResult = Result<JsonNode, &'static str>;

fn get_json_node(s: &String, is_string: Option<bool>) -> DecodeJsonResult {
    let trimmed_s = String::from(s.trim());
    let cast_to_string = is_string.unwrap_or(false); // default parse to number
    match trimmed_s.parse::<f64>() {
        Ok(n) if !cast_to_string => Ok(JsonNode::Number(n)),
        Ok(_) => Ok(JsonNode::String(String::from(s))),
        Err(_) if cast_to_string => Ok(JsonNode::String(String::from(s))),
        Err(_) => Err("failed to cast string to number"),
    }
}

fn lrtrim_json(json: &String) -> String {
    json[1..json.len() - 1].trim().to_string()
}

fn is_whole(json: &String) -> bool {
    let mut stack: VecDeque<StackItem> = VecDeque::new();
    for c in json.chars() {
        let pop = stack.back().unwrap_or(&StackItem::Empty);
        match c {
            '{' => stack.push_back(StackItem::Object(false)),
            '[' => stack.push_back(StackItem::Array(false)),
            '}' => {
                if pop == &StackItem::Object(false) {
                    stack.pop_back();
                } else {
                    return false;
                }
            }
            ']' => {
                if pop == &StackItem::Array(false) {
                    stack.pop_back();
                } else {
                    return false;
                }
            }
            _ => {}
        };
    }
    if stack.len() > 0 {
        false
    } else {
        true
    }
}

fn decode_object(json: String) -> DecodeJsonResult {
    let mut stack: VecDeque<StackItem> = VecDeque::new();
    let mut decode_buf = String::new();
    let mut cur_buf = String::new();
    let mut key = String::new();
    let mut result: HashMap<String, JsonNode> = HashMap::new();
    let mut is_value_string: Option<bool> = None;

    for (i, c) in json.chars().enumerate() {
        let pop = stack.back().unwrap_or(&StackItem::Empty);
        match c {
            '{' if decode_buf.is_empty() => {
                stack.push_back(StackItem::Object(false));
                decode_buf.push(c);
            }
            '[' if decode_buf.is_empty() => {
                stack.push_back(StackItem::Array(false));
                decode_buf.push(c);
            }
            '}' | ']' if !decode_buf.is_empty() => {
                decode_buf.push(c);

                if is_whole(&decode_buf) {
                    stack.pop_back();
                    let node = decode_binding(&decode_buf)?;
                    let pop = stack.back().unwrap_or(&StackItem::Empty);
                    if pop == &StackItem::Key(true) {
                        result.insert(String::from(&key), node);
                        key.clear();
                        decode_buf.clear();
                        stack.clear();
                    } else {
                        return Err("stack last item must be a key");
                    }
                }
            }
            ':' if decode_buf.is_empty() => {
                cur_buf.clear();
            }
            '"' if decode_buf.is_empty() => match &pop {
                StackItem::Empty => {
                    stack.push_back(StackItem::Key(false));
                    cur_buf.clear();
                }
                StackItem::Key(false) => {
                    key = String::from(&cur_buf);
                    cur_buf.clear();
                    stack.push_back(StackItem::Key(true));
                }
                StackItem::Key(true) => {
                    cur_buf.clear();
                    stack.push_back(StackItem::Value(false));
                    is_value_string = Some(true);
                }
                StackItem::Value(false) => {
                    let node = get_json_node(&cur_buf, is_value_string)?;
                    result.insert(String::from(&key), node);
                    key.clear();
                    cur_buf.clear();
                    is_value_string = None;
                }
                _ => {
                    return Err("detected invalid double quote");
                }
            },
            ',' if decode_buf.is_empty() => {
                if pop == &StackItem::Key(true) && !cur_buf.is_empty() {
                    let node = get_json_node(&cur_buf, is_value_string)?;
                    result.insert(String::from(&key), node);
                    key.clear();
                    cur_buf.clear();
                    is_value_string = None;
                }
                stack.clear();
            }
            _ => {
                if decode_buf.is_empty() {
                    cur_buf.push(c);
                } else {
                    decode_buf.push(c);
                }

                if i == json.len() - 1 && pop == &StackItem::Key(true) && !cur_buf.is_empty() {
                    let node = get_json_node(&cur_buf, is_value_string)?;
                    result.insert(String::from(&key), node);
                    is_value_string = None;
                }
            }
        }
    }
    Ok(JsonNode::Object(result))
}

fn decode_array(json: String) -> DecodeJsonResult {
    let mut stack: VecDeque<StackItem> = VecDeque::new();
    let mut decode_buf = String::new();
    let mut cur_buf = String::new();
    let mut result: Vec<JsonNode> = Vec::new();
    let mut is_value_string: Option<bool> = None;

    for (i, c) in json.chars().enumerate() {
        match c {
            '{' if decode_buf.is_empty() => {
                stack.push_back(StackItem::Object(false));
                decode_buf.push(c);
            }
            '[' if decode_buf.is_empty() => {
                stack.push_back(StackItem::Array(false));
                decode_buf.push(c);
            }
            '}' | ']' if !decode_buf.is_empty() => {
                decode_buf.push(c);

                if is_whole(&decode_buf) {
                    stack.pop_back();
                    let node = decode_binding(&decode_buf)?;
                    let pop = stack.back().unwrap_or(&StackItem::Empty);
                    if pop == &StackItem::Empty {
                        result.push(node);
                        decode_buf.clear();
                        stack.clear();
                    } else {
                        return Err("stack last item must be a key");
                    }
                }
            }
            '"' if decode_buf.is_empty() => {
                let pop = stack.back().unwrap_or(&StackItem::Empty);
                match &pop {
                    StackItem::Empty => {
                        stack.push_back(StackItem::Value(false));
                        is_value_string = Some(true);
                        cur_buf.clear();
                    }
                    StackItem::Value(false) => {
                        stack.pop_back();
                    }
                    _ => return Err("array stack not empty"),
                }

                if i == json.len() - 1 {
                    let node = get_json_node(&cur_buf, is_value_string)?;
                    result.push(node);
                    is_value_string = None;
                }
            }
            ',' if decode_buf.is_empty() => {
                if !is_value_string.unwrap_or(false) && cur_buf.is_empty() {
                    continue;
                }
                let node = get_json_node(&cur_buf, is_value_string)?;
                result.push(node);
                cur_buf.clear();
                is_value_string = None;
            }
            _ => {
                if decode_buf.is_empty() {
                    cur_buf.push(c);
                } else {
                    decode_buf.push(c);
                }
                if i == json.len() - 1 {
                    let node = get_json_node(&cur_buf, is_value_string)?;
                    result.push(node);
                    is_value_string = None;
                }
            }
        };
    }
    Ok(JsonNode::Array(result))
}

fn decode_binding(json: &String) -> DecodeJsonResult {
    let first_char = match json.chars().next() {
        Some(c) => c,
        None => return Err("failed to get first character"),
    };
    let last_char = match json.chars().last() {
        Some(c) => c,
        None => return Err("failed to get last character"),
    };
    if first_char == '{' && last_char == '}' {
        decode_object(lrtrim_json(&json))
    } else if first_char == '[' && last_char == ']' {
        decode_array(lrtrim_json(&json))
    } else {
        return Err("first character and last character are not the same type");
    }
}

pub fn decode_json(json: &String) -> DecodeJsonResult {
    if !is_whole(json) {
        return Err("invalid json string");
    } else if json.contains("\\") {
        return Err("invalid json string '\\' not allowed");
    }

    let json = String::from(json.trim());
    decode_binding(&json)
}

pub fn decode_json_from_bytes(json_bytes: &Vec<u8>) -> DecodeJsonResult {
    match std::str::from_utf8(json_bytes) {
        Ok(s) => decode_json(&s.to_string()),
        Err(_) => Err("failed to convert byte array to utf8 json string"),
    }
}
