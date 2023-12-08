use json_parser::{encode::encode_json, JsonNode};
use logger;
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    net::TcpStream,
    str,
};

use super::utils::{format_response, HttpResponseCode};

pub fn html_response(
    stream: &TcpStream,
    html_file_name: &str,
    http_response_code: &HttpResponseCode,
) {
    let contents = match fs::read_to_string(html_file_name) {
        Ok(s) => s,
        Err(err) => {
            logger::log!(error "failed to read file \"{}\": {:?}", html_file_name, err);
            let content = "".to_string();
            let response = format_response(&content, http_response_code, &HashMap::new());
            crate::write_http_response!(&stream, response.as_bytes());
            return;
        }
    };
    let response = format_response(&contents, http_response_code, &HashMap::new());
    crate::write_http_response!(&stream, response.as_bytes());
}

pub fn http_response(
    stream: &TcpStream,
    http_response_body: &JsonNode,
    http_response_code: &HttpResponseCode,
) {
    let json_string = encode_json(http_response_body);
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("Accept".to_string(), "application/json".to_string());
    let response = format_response(&json_string, http_response_code, &headers);
    crate::write_http_response!(&stream, response.as_bytes());
}

#[macro_export]
macro_rules! html_response {
    ($stream:expr, $html_file_name:expr) => {
        crate::http_parser::response::html_response(
            $stream,
            $html_file_name,
            &crate::http_parser::utils::HttpResponseCode::OK,
        )
    };
    ($stream:expr, $html_file_name:expr, $http_response_code:expr) => {
        crate::http_parser::response::html_response($stream, $html_file_name, $http_response_code)
    };
    (error $stream:expr) => {
        crate::http_parser::response::html_response(
            $stream,
            "./static/500.html",
            500,
            "INTERNAL SERVER ERROR",
        );
    };
}

#[macro_export]
macro_rules! http_response {
    ($stream:expr, $message:expr, $http_response_code:expr) => {
        use json_parser::JsonNode;
        use std::collections::HashMap;
        let mut o: HashMap<String, JsonNode> = HashMap::new();
        o.insert(
            "message".to_string(),
            JsonNode::String($message.to_string()),
        );

        crate::http_parser::response::http_response(
            $stream,
            &JsonNode::Object(o),
            $http_response_code,
        );
    };
}

pub fn write_http_response(mut stream: &TcpStream, response_buf_vec: Vec<&[u8]>) -> io::Result<()> {
    for response_buf in response_buf_vec {
        stream.write_all(response_buf)?;
    }
    stream.flush()?;
    Ok(())
}

#[macro_export]
macro_rules! write_http_response {
    ( $stream:expr, $( $response_buf:expr ), * ) => {{
        if let Err(err) =  crate::http_parser::response::write_http_response($stream, vec![ $( $response_buf ), * ]) {
            logger::log!(error "failed to write http response {:?}", err);
        }
    }};
}
