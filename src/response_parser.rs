use logger;
use std::{
    env::join_paths,
    fs,
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
};

pub struct HttpFormatter {
    request_protocol: String,
}

impl HttpFormatter {
    pub fn new(request_protocol: String) -> Self {
        HttpFormatter { request_protocol }
    }

    pub fn format_response(&self, contents: &String, code: u16, status: &str) -> String {
        let status_line = format!("{} {code} {status}", self.request_protocol);
        let content_length = contents.len();
        format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{contents}")
    }

    pub fn get_request_protocol(&self) -> String {
        String::from(&self.request_protocol)
    }
}

pub fn html_response_marco(
    http_formatter: &HttpFormatter,
    stream: &TcpStream,
    html_file_name: &str,
    code: u16,
    status: &str,
) {
    let contents = match fs::read_to_string(html_file_name) {
        Ok(s) => s,
        Err(err) => {
            logger::log!(error "failed to read file \"{}\": {:?}", html_file_name, err);
            let content = "".to_string();
            let response = http_formatter.format_response(&content, 500, "INTERNAL SERVER ERROR");
            crate::write_http_response!(&stream, response.as_bytes());
            return;
        }
    };
    let response = http_formatter.format_response(&contents, code, status);
    crate::write_http_response!(&stream, response.as_bytes());
}

#[macro_export]
macro_rules! html_response {
    ($http_formatter:expr, $stream:expr, $html_file_name:expr) => {
        crate::response_parser::html_response_marco(
            $http_formatter,
            $stream,
            $html_file_name,
            200,
            "OK",
        );
    };
    ($http_formatter:expr, $stream:expr, $html_file_name:expr, $code:expr, $status:expr) => {
        crate::response_parser::html_response_marco(
            $http_formatter,
            $stream,
            $html_file_name,
            $code,
            $status,
        );
    };
}

#[macro_export]
macro_rules! error_response {
    (html $http_formatter:expr, $stream:expr) => {
        crate::response_parser::html_response_marco(
            $http_formatter,
            $stream,
            "./static/500.html",
            500,
            "INTERNAL SERVER ERROR",
        );
    };
    (http $http_formatter:expr, $stream:expr) => {
        crate::response_parser::html_response_marco(
            $http_formatter,
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

pub fn get_http_request_headers(buf_reader: BufReader<&mut TcpStream>) -> Vec<(String, String)> {
    let mut is_first_line = true;
    let mut is_headers_end = false;
    let mut other = String::new();
    let mut request_headers: Vec<(String, String)> = Vec::new();
    let _request_lines: Vec<String> = buf_reader
        .lines()
        .map(|result| {
            result.unwrap()
            // let result = result.unwrap();
            // if is_first_line {
            //     let request_info: Vec<&str> = result.split(" ").collect();
            //     request_headers.push(("Method".to_string(), request_info[0].to_string()));
            //     request_headers.push(("Path".to_string(), request_info[1].to_string()));
            //     request_headers.push(("Protocol".to_string(), request_info[2].to_string()));
            //     is_first_line = false;
            // } else if !is_headers_end {
            //     if result.contains(": ") {
            //         let request_vec: Vec<&str> = result.split(": ").collect();
            //         request_headers.push((request_vec[0].to_string(), request_vec[1].to_string()));
            //     } else {
            //         is_headers_end = true;
            //     }
            // } else {
            //     other.push_str(&result);
            // }
            // result
        })
        .collect();
    // for line in buf_reader.lines() {
    //     match line {
    //         Ok(line) if is_first_line => {
    //             let request_info: Vec<&str> = line.split(" ").collect();
    //             request_headers.push(("Method".to_string(), request_info[0].to_string()));
    //             request_headers.push(("Path".to_string(), request_info[1].to_string()));
    //             request_headers.push(("Protocol".to_string(), request_info[2].to_string()));
    //             is_first_line = false;
    //         }
    //         Ok(line) if !is_headers_end => {
    //             if line.contains(": ") {
    //                 let request_vec: Vec<&str> = line.split(": ").collect();
    //                 request_headers.push((request_vec[0].to_string(), request_vec[1].to_string()));
    //             } else {
    //                 is_headers_end = true;
    //             }
    //         }
    //         Ok(line) => {
    //             other.push_str(&line);
    //         }
    //         Err(err) => {
    //             logger::log!(error "failed to read http request line: {:?}", err);
    //             return Vec::new()
    //         }
    //     }
    // }

    println!("{:?}", request_headers);
    request_headers
}
