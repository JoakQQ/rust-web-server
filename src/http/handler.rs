use std::net::TcpStream;

use crate::http_parser::request::parse_request;

use super::{route::handle_http_response, HttpResult};

pub fn handle_connection(stream: TcpStream) -> HttpResult {
    match parse_request(&stream) {
        Ok((headers, request_body)) => {
            return handle_http_response(stream, headers, request_body);
        },
        Err(err) => {
            logger::log!(error "{:?}", err);
            return Err("");
        },
    };
}
