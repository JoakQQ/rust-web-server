use std::{fs::File, io::Read, net::TcpStream};

use crate::{error_response, response_parser, write_http_response};

pub fn favicon_response(http_formatter: &response_parser::HttpFormatter, stream: TcpStream) {
    let icon_path = "./static/images/favicon.ico";
    match File::open(icon_path) {
        Ok(mut file) => {
            let mut icon_buf: Vec<u8> = Vec::new();
            if let Err(err) = file.read_to_end(&mut icon_buf) {
                logger::log!(error "failed to read file \"{}\": {:?}", icon_path, err);
                error_response!(http & http_formatter, &stream);
            }
            let headers = [
                format!("{} 200 OK", http_formatter.get_request_protocol()),
                format!("Content-Length: {}", icon_buf.len()),
                String::from("Content-Type: image/x-icon"),
                String::from("\r\n"),
            ];
            write_http_response!(&stream, headers.join("\r\n").as_bytes(), &icon_buf);
        }
        Err(err) => {
            logger::log!(error "failed to open file \"{}\": {:?}", icon_path, err);
            error_response!(http & http_formatter, &stream);
        }
    };
}
