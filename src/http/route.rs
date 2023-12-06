use std::net::TcpStream;

use crate::{html_response, response_parser};

use super::favicon;

pub fn handle_http_response(stream: TcpStream, request_info: &Vec<&str>) {
    let request_method = request_info[0];
    let request_path = request_info[1];
    let request_protocol = request_info[2];

    let http_formatter = response_parser::HttpFormatter::new(request_protocol.to_string());

    logger::log!(info "path {}", request_path);

    match request_method {
        "GET" => match request_path {
            "/" => {
                // index page
                html_response!(&http_formatter, &stream, "./static/index.html");
            }
            "/favicon.ico" => {
                // favicon
                favicon::favicon_response(&http_formatter, stream);
            }
            _ => {
                // not found response
                html_response!(
                    &http_formatter,
                    &stream,
                    "./static/404.html",
                    404,
                    "NOT FOUND"
                );
            }
        },
        _ => {
            // method not allowed response
            html_response!(
                &http_formatter,
                &stream,
                "./static/405.html",
                405,
                "METHOD NOT ALLOWED"
            );
        }
    }
}
