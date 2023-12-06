use std::{net::{TcpListener, TcpStream}, io::{BufReader, prelude::*}};
use web_server::{http::{formatter, http_macro, thread}, html_response};
use logger;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let thread_pool = thread::ThreadPool::new(4);

    // logger::set_logger_env(logger::LoggerEnv::PROD);
    println!("using logger environment {:?}", logger::get_logger_env());

    logger::log!(info "listening to incoming http requests");
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                thread_pool.execute(|| {
                    handle_connection(stream);
                });
            },
            Err(err) => {
                logger::log!(error "{:?}", err);
            },
        }
    }

    logger::log!(info "shutting down web server");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut http_request_lines = buf_reader.lines();
    let first_http_request_line = match http_request_lines.next() {
        Some(line) => {
            match line {
                Ok(l) => l,
                Err(err) => {
                    logger::log!(error "error getting first line: {:?}", err);
                    return
                },
            }
        },
        None => return,
    };
    let info_vec: Vec<&str> = first_http_request_line.split(" ").collect();
    if info_vec.len() >=3 {
        handle_http_response(stream, &info_vec);
    }
}

fn handle_http_response(stream: TcpStream, request_info: &Vec<&str>) {
    let request_method = request_info[0];
    let request_path = request_info[1];
    let request_protocol = request_info[2];

    let http_formatter = formatter::HttpFormatter::new(request_protocol);

    match request_method {
        "GET" => match request_path {
            "/" => html_response!(http_formatter, stream, "./static/index.html"),
            _ => html_response!(http_formatter, stream, "./static/404.html", 404, "NOT FOUND"),
        },
        _ => { html_response!(http_formatter, stream, "./static/405.html", 405, "METHOD NOT ALLOWED"); },
    }
}
