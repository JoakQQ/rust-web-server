use logger;
use std::{
    collections::HashMap,
    fmt, fs,
    io::{self, Read, Write},
    net::TcpStream,
    str,
};

static mut MAX_HEADER_SIZE: usize = 1024;

pub fn format_response(contents: &String, code: u16, status: &str) -> String {
    let status_line = format!("HTTP/1.1 {code} {status}");
    let content_length = contents.len();
    format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}")
}

pub fn html_response_marco(stream: &TcpStream, html_file_name: &str, code: u16, status: &str) {
    let contents = match fs::read_to_string(html_file_name) {
        Ok(s) => s,
        Err(err) => {
            logger::log!(error "failed to read file \"{}\": {:?}", html_file_name, err);
            let content = "".to_string();
            let response = format_response(&content, 500, "INTERNAL SERVER ERROR");
            crate::write_http_response!(&stream, response.as_bytes());
            return;
        }
    };
    let response = format_response(&contents, code, status);
    crate::write_http_response!(&stream, response.as_bytes());
}

#[macro_export]
macro_rules! html_response {
    ($stream:expr, $html_file_name:expr) => {
        crate::response_parser::html_response_marco($stream, $html_file_name, 200, "OK")
    };
    ($stream:expr, $html_file_name:expr, $code:expr, $status:expr) => {
        crate::response_parser::html_response_marco($stream, $html_file_name, $code, $status)
    };
}

#[macro_export]
macro_rules! error_response {
    (html $stream:expr) => {
        crate::response_parser::html_response_marco(
            $stream,
            "./static/500.html",
            500,
            "INTERNAL SERVER ERROR",
        );
    };
    (http $stream:expr) => {
        crate::response_parser::html_response_marco(
            $stream,
            "./static/500.html",
            500,
            "INTERNAL SERVER ERROR",
        );
    };
}

pub fn write_http_response(
    mut stream: &TcpStream,
    response_buf_vec: Vec<&[u8]>,
) -> io::Result<String> {
    for response_buf in response_buf_vec {
        stream.write_all(response_buf)?;
    }
    stream.flush()?;
    Ok(String::new())
}

#[macro_export]
macro_rules! write_http_response {
    ( $stream:expr, $( $response_buf:expr ), * ) => {{
        if let Err(err) =  crate::response_parser::write_http_response($stream, vec![ $( $response_buf ), * ]) {
            logger::log!(error "failed to write http response {:?}", err);
        }
    }};
}

#[derive(Debug, Clone)]
pub struct ParseHttpRequestError;

impl fmt::Display for ParseHttpRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse http request")
    }
}

pub fn parse_http_request(
    mut stream: &TcpStream,
) -> Result<(HashMap<String, String>, String), ParseHttpRequestError> {
    let mut halt = false;
    let mut is_first_line = true;
    let mut is_headers_end = false;
    let mut request_body = String::new();
    let mut headers: HashMap<String, String> = HashMap::new();
    let mut cur_header_size: usize = 0;

    loop {
        let mut cur_buf = [0; 1024];
        if let Err(err) = stream.read(&mut cur_buf) {
            logger::log!(error "failed to read stream: {:?}", err);
        }

        if halt {
            continue;
        }
        if cur_header_size > unsafe { MAX_HEADER_SIZE } {
            halt = true;
        }

        match str::from_utf8(&cur_buf) {
            Ok(lines) => {
                let lines: Vec<&str> = lines.split("\r\n").collect();
                for line in lines {
                    if is_first_line {
                        if !line.contains(" ") {
                            halt = true;
                            continue;
                        }

                        cur_header_size += line.len();
                        is_first_line = false;

                        let mut line_iter = line.split(" ");
                        headers.insert("Method".to_string(), line_iter.next().unwrap().to_string());
                        headers.insert("Path".to_string(), line_iter.next().unwrap().to_string());
                        headers.insert(
                            "Protocol".to_string(),
                            line_iter.next().unwrap().to_string(),
                        );
                    } else if !is_headers_end {
                        if line.contains(": ") {
                            cur_header_size += line.len();

                            let mut line_iter = line.split(": ");
                            headers.insert(
                                line_iter.next().unwrap().to_string(),
                                line_iter.next().unwrap().to_string(),
                            );
                        } else {
                            is_headers_end = true;
                        }
                    } else {
                        request_body.push_str(line);
                    }
                }
            }
            Err(_) => {}
        }
        if cur_buf[1023] == 0 {
            break;
        }
    }

    if halt {
        let response_headers = [
            String::from("HTTP/1.1 431 REQUEST HEADER FIELDS TOO LARGE"),
            String::from("\r\n"),
        ];
        write_http_response!(&stream, response_headers.join("\r\n").as_bytes());
        return Err(ParseHttpRequestError);
    }
    Ok((headers, request_body))
}
