use std::{fs::File, io::Read, net::TcpStream};

use crate::{http_response, write_http_response, http_parser::utils::HttpResponseCode};

pub fn favicon_response(stream: TcpStream) {
    let icon_path = "./static/images/favicon.ico";
    match File::open(icon_path) {
        Ok(mut file) => {
            let mut icon_buf: Vec<u8> = Vec::new();
            if let Err(err) = file.read_to_end(&mut icon_buf) {
                logger::log!(error "failed to read file \"{}\": {:?}", icon_path, err);
                http_response!(&stream, "failed to read file", &HttpResponseCode::InternalServerError);
            }
            let headers = [
                String::from("HTTP/1.1 200 OK"),
                format!("Content-Length: {}", icon_buf.len()),
                String::from("Content-Type: image/x-icon"),
                String::from("\r\n"),
            ];
            write_http_response!(&stream, headers.join("\r\n").as_bytes(), &icon_buf);
        }
        Err(err) => {
            logger::log!(error "failed to open file \"{}\": {:?}", icon_path, err);
            http_response!(&stream, "failed to open file", &HttpResponseCode::InternalServerError);
        }
    };
}
