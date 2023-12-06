use std::{io::{BufReader, BufRead}, net::TcpStream};

use crate::response_parser::get_http_request_headers;

use super::route::handle_http_response;

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request_headers = get_http_request_headers(buf_reader);
    // let mut request_method = String::new();
    // let mut request_path = String::new();
    // let mut request_protocol = String::new();
    // for (header_name, header_value) in &http_request_headers {
    //     match header_name.as_str() {
    //         "Method" | "Path" | "Protocol" => request_method = String::from(header_value),
    //         _ => {},
    //     }
    // }

    let mut http_request_lines = buf_reader.lines();
    let first_http_request_line = match http_request_lines.next() {
        Some(line) => match line {
            Ok(l) => l,
            Err(err) => {
                logger::log!(error "error getting first line: {:?}", err);
                return;
            }
        },
        None => return,
    };
    let info_vec: Vec<&str> = first_http_request_line.split(" ").collect();
    if info_vec.len() >= 3 {
        handle_http_response(stream, &info_vec);
    }
}