use crate::JsonNode;

pub fn encode_json(node: &JsonNode) -> String {
    match &node {
        JsonNode::Number(n) => return n.to_string(),
        JsonNode::String(s) => {
            let mut result: String = String::from('"');
            result.push_str(s);
            result.push('"');
            result
        }
        JsonNode::Object(o) => {
            let mut result: String = String::from('{');
            let nodes: Vec<String> = o
                .iter()
                .map(|(key, value)| format!("\"{key}\": {}", encode_json(value)))
                .collect();
            result.push_str(nodes.join(", ").as_str());
            result.push('}');
            result
        }
        JsonNode::Array(a) => {
            let mut result: String = String::from('[');
            let nodes: Vec<String> = a.iter().map(|node| encode_json(node)).collect();
            result.push_str(nodes.join(", ").as_str());
            result.push(']');
            result
        }
    }
}
