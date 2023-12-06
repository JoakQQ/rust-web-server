use logger;
use std::net::TcpListener;
use web_server::{
    thread_pool, http::handler::handle_connection,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let thread_pool = thread_pool::ThreadPool::new(4);

    // logger::set_logger_env(logger::LoggerEnv::DEBUG);
    println!("using logger environment {:?}", logger::get_logger_env());

    logger::log!(info "listening to incoming http requests");
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                thread_pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(err) => {
                logger::log!(error "{:?}", err);
            }
        }
    }

    logger::log!(info "shutting down web server");
}
