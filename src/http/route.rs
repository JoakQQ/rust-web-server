use std::{collections::HashMap, net::TcpStream};

use json_parser::JsonNode;

use crate::{
    html_response,
    http_parser::{
        response::http_response,
        utils::HttpResponseCode,
    },
    http_response,
};

use super::{favicon, HttpResult};

pub fn handle_http_response(
    stream: TcpStream,
    headers: HashMap<String, String>,
    request_body: Vec<u8>,
) -> HttpResult {
    let request_method = match headers.get(&"Method".to_string()) {
        Some(m) => m,
        None => return Err("failed to get \"Method\" from header"),
    };
    let request_path = match headers.get(&"Path".to_string()) {
        Some(p) => p,
        None => return Err("failed to get \"Path\" from header"),
    };

    logger::log!(info "{} {} {}",
        (headers.get(&"Host".to_string()).unwrap_or(&"-".to_string())),
        request_method,
        request_path
    );

    match request_method.as_str() {
        "GET" => route_get_requests(stream, &request_path, &headers),
        "POST" => route_post_requests(stream, &request_path, &headers, &request_body),
        _ => route_default(stream, &request_path, &headers, &request_body),
    };
    Ok(())
}

fn route_get_requests(
    stream: TcpStream,
    request_path: &String,
    _headers: &HashMap<String, String>,
) {
    match request_path.as_str() {
        "/" => {
            // index page
            html_response!(&stream, "./static/index.html");
        }
        "/favicon.ico" => {
            // favicon
            favicon::favicon_response(stream);
        }
        _ => {
            // not found response
            html_response!(&stream, "./static/404.html", &HttpResponseCode::NotFound);
        }
    }
}

fn route_post_requests(
    stream: TcpStream,
    request_path: &String,
    headers: &HashMap<String, String>,
    request_body: &Vec<u8>,
) {
    match headers.get("Content-Type") {
        Some(content_tpye) => {
            if content_tpye.ne("application/json") {
                http_response!(
                    &stream,
                    "header \"Content-Type\" is not \"application/json\"",
                    &HttpResponseCode::MethodNotAlloed
                );
                return;
            }
        }
        None => {
            http_response!(
                &stream,
                "consider adding \"application/json\" to header\"Content-Type\"",
                &HttpResponseCode::MethodNotAlloed
            );
            return;
        }
    };
    match request_path.as_str() {
        "/" => {
            // index page
            let node = match json_parser::decode::decode_json_from_bytes(request_body) {
                Ok(node) => node,
                Err(err) => {
                    http_response!(&stream, err, &HttpResponseCode::BadRequest);
                    return;
                }
            };
            let mut response: HashMap<String, JsonNode> = HashMap::new();
            response.insert(
                "message".to_string(),
                JsonNode::String("received input".to_string()),
            );
            response.insert("input".to_string(), node);
            http_response(&stream, &JsonNode::Object(response), &HttpResponseCode::OK);
        }
        "/ping" => {
            http_response!(&stream, "pong", &HttpResponseCode::OK);
        }
        _ => {
            // not found response
            http_response!(&stream, "not found", &HttpResponseCode::NotFound);
        }
    }
}

fn route_default(
    stream: TcpStream,
    _request_path: &String,
    _headers: &HashMap<String, String>,
    _request_body: &Vec<u8>,
) {
    html_response!(
        &stream,
        "./static/405.html",
        &HttpResponseCode::MethodNotAlloed
    );
}
