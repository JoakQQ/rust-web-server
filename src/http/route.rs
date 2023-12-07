use std::net::TcpStream;

use crate::html_response;

use super::favicon;

pub fn handle_http_response(
    stream: TcpStream,
    headers: Vec<(String, String)>,
    request_body: String,
) {
    let mut request_method = String::new();
    let mut request_path = String::new();

    headers.iter().for_each(|(key, value)| match key.as_str() {
        "Method" => request_method = String::from(value),
        "Path" => request_path = String::from(value),
        _ => {}
    });

    match request_method.as_str() {
        "GET" => route_get_requests(stream, &request_path, &headers),
        _ => route_default(stream, &request_path, &headers, request_body),
    }
}

fn route_get_requests(stream: TcpStream, request_path: &String, _headers: &Vec<(String, String)>) {
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
    _headers: &Vec<(String, String)>,
    _request_body: String,
) {
    html_response!(&stream, "./static/405.html", 405, "METHOD NOT ALLOWED");
}
