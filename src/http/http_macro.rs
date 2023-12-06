use std::{net::TcpStream, fs, io::prelude::*};
use crate::http::formatter;
use logger;

fn write_http_response(mut stream: TcpStream, response_buf: &[u8]) {
    match stream.write_all(response_buf) {
        Ok(_) => logger::log!(info "sent response to client"),
        Err(err) => logger::log!(error "failed to send response to client: {:?}", err),
    }
}

pub fn html_response_marco(http_formatter: formatter::HttpFormatter, stream: TcpStream, html_file_name: &str, code: u16, status: &str) {
    let contents = match fs::read_to_string(html_file_name) {
        Ok(s) => s,
        Err(_) => return,
    };
    let response = http_formatter.format_response(&contents, code, status);
    write_http_response(stream, response.as_bytes());
}

#[macro_export]
macro_rules! html_response {
    ($http_formatter:expr, $stream:expr, $html_file_name:expr) => {
        crate::http_macro::html_response_marco($http_formatter, $stream, $html_file_name, 200, "OK");
    };
    ($http_formatter:expr, $stream:expr, $html_file_name:expr, $code:expr, $status:expr) => {
        crate::http_macro::html_response_marco($http_formatter, $stream, $html_file_name, $code, $status);
    };
}
