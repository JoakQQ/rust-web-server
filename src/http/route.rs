use std::{net::TcpStream, collections::HashMap};

use crate::html_response;

use super::{favicon, HttpResult};

pub fn handle_http_response(
    stream: TcpStream,
    headers: HashMap<String, String>,
    request_body: String,
) -> HttpResult {
    let request_method = match headers.get(&"Method".to_string()) {
        Some(m) => m,
        None => return Err("failed to get \"Method\" from header"),
    };
    let request_path = match headers.get(&"Path".to_string()) {
        Some(p) => p,
        None => return Err("failed to get \"Path\" from header"),
    };

    match request_method.as_str() {
        "GET" => route_get_requests(stream, &request_path, &headers),
        _ => route_default(stream, &request_path, &headers, request_body),
    };
    Ok(())
}

fn route_get_requests(stream: TcpStream, request_path: &String, _headers: &HashMap<String, String>) {
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
            html_response!(&stream, "./static/404.html", 404, "NOT FOUND");
        }
    }
}

fn route_default(
    stream: TcpStream,
    _request_path: &String,
    _headers: &HashMap<String, String>,
    _request_body: String,
) {
    html_response!(&stream, "./static/405.html", 405, "METHOD NOT ALLOWED");
}
