pub mod response_parser;
pub mod thread_pool;

pub mod http {
	pub mod handler;
	pub mod route;
	pub mod favicon;

	pub type HttpResult = std::result::Result<(), &'static str>;
}