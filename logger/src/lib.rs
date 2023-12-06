static mut LOGGER_ENVIRONMENT: LoggerEnv = LoggerEnv::DEV;

/// logger level
pub enum LoggerLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

pub fn log_macro(level: LoggerLevel, message: String) {
    println!(
        "{{ \"level\": \"{}\", \"message\": \"{}\" }}",
        unsafe {
            match level {
                LoggerLevel::DEBUG if LOGGER_ENVIRONMENT != LoggerEnv::DEBUG => return,
                LoggerLevel::DEBUG => "debug",
                LoggerLevel::INFO => "info",
                LoggerLevel::WARN => "warn",
                LoggerLevel::ERROR => "error",
            }
        },
        message,
    );
}

/// logger environment
#[derive(Debug, PartialEq)]
pub enum LoggerEnv {
    DEBUG,
    DEV,
    UAT,
    PROD,
}

pub fn set_logger_env(env: LoggerEnv) {
    unsafe {
        LOGGER_ENVIRONMENT = env;
    }
}

pub fn get_logger_env() -> &'static LoggerEnv {
    unsafe { &LOGGER_ENVIRONMENT }
}

/// custom logger log function
#[macro_export]
macro_rules! log {
	( debug $( $arg:tt ), * ) => {{
		logger::log_macro(logger::LoggerLevel::DEBUG, format!($($arg),*));
	}};
	( info $( $arg:tt ), * ) => {{
		logger::log_macro(logger::LoggerLevel::INFO, format!($($arg),*));
	}};
	( warn $( $arg:tt ), * ) => {{
		logger::log_macro(logger::LoggerLevel::WARN, format!($($arg),*));
	}};
	( error $( $arg:tt ), * ) => {{
		logger::log_macro(logger::LoggerLevel::ERROR, format!($($arg),*));
	}};
}
