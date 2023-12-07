use logger;
use std::net::TcpListener;
use web_server::{
    thread_pool, http::handler::handle_connection,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let thread_pool = thread_pool::ThreadPool::new(4);

    logger::set_logger_level(logger::LoggerLevel::INFO);
    println!("using logger level {}", logger::get_logger_level());

    logger::log!(info "listening to incoming http requests");
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                thread_pool.execute(|| {
                    if let Err(err) = handle_connection(stream) {
                        logger::log!(error "{}", err);
                    }
                });
            }
            Err(err) => {
                logger::log!(error "{:?}", err);
            }
        }
    }

    logger::log!(info "shutting down web server");
}
