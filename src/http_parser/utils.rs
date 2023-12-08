use std::collections::HashMap;

pub enum HttpResponseCode {
    OK,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAlloed,
    RequestHeaderFieldsTooLarge,
    InternalServerError,
}

pub fn get_response_info(http_response_code: &HttpResponseCode) -> (u16, &'static str) {
    match &http_response_code {
        HttpResponseCode::OK => (200, "OK"),
        HttpResponseCode::BadRequest => (400, "Bad Request"),
        HttpResponseCode::Unauthorized => (401, "Unauthorized"),
        HttpResponseCode::Forbidden => (403, "Forbidden"),
        HttpResponseCode::NotFound => (404, "Not Found"),
        HttpResponseCode::MethodNotAlloed => (415, "Method Not Alloed"),
        HttpResponseCode::RequestHeaderFieldsTooLarge => (431, "Request Header Fields Too Large"),
        HttpResponseCode::InternalServerError => (500, "Internal Server Error"),
    }
}

pub fn format_response(
    contents: &String,
    http_response_code: &HttpResponseCode,
    headers: &HashMap<String, String>,
) -> String {
    let (code, status) = get_response_info(http_response_code);
    let status_line = format!("HTTP/1.1 {code} {status}");
    let content_length = contents.len() + 2;
    let mut response_lines = vec![status_line, format!("Content-Length: {content_length}")];
    headers.iter().for_each(|(key, value)| {
        response_lines.push(format!("{key}: {value}"));
    });
    response_lines.push("\r\n".to_string());
    response_lines.push(String::from(contents));
    response_lines.join("\r\n")
}
