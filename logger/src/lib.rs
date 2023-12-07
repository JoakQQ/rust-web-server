static mut LOGGER_LEVEL: LoggerLevel = LoggerLevel::INFO;

/// logger level
#[derive(PartialEq, PartialOrd, Debug)]
pub enum LoggerLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl std::fmt::Display for LoggerLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "logger::LoggerLevel::{:?}", self)
    }
}

pub fn log_macro(level: LoggerLevel, message: String, extra_fields: Vec<(&str, &str)>) {
    let can_print: bool = &level >= unsafe { &LOGGER_LEVEL };
    if can_print {
        let mut print_stack: Vec<String> = vec![
            format!(
                "\"level\": \"{}\"",
                match level {
                    LoggerLevel::DEBUG => "debug",
                    LoggerLevel::INFO => "info",
                    LoggerLevel::WARN => "warn",
                    LoggerLevel::ERROR => "error",
                }
            ),
            format!("\"message\": \"{}\"", message),
        ];
        extra_fields.iter().for_each(|&(key, value)| {
            print_stack.push(format!("\"{key}\": \"{value}\""));
        });
        println!("{{ {} }}", print_stack.join(", "));
    }
}

pub fn set_logger_level(level: LoggerLevel) {
    unsafe {
        LOGGER_LEVEL = level;
    }
}

pub fn get_logger_level() -> &'static LoggerLevel {
    unsafe { &LOGGER_LEVEL }
}

/// custom logger log function
#[macro_export]
macro_rules! log {
	( debug $( $arg:tt ), * ) => {{
		logger::log_macro(logger::LoggerLevel::DEBUG, format!($($arg),*), vec![]);
	}};
	( info $( $arg:tt ), * ) => {{
		logger::log_macro(logger::LoggerLevel::INFO, format!($($arg),*), vec![]);
	}};
	( warn $( $arg:tt ), * ) => {{
		logger::log_macro(logger::LoggerLevel::WARN, format!($($arg),*), vec![]);
	}};
	( error $( $arg:tt ), * ) => {{
		logger::log_macro(logger::LoggerLevel::ERROR, format!($($arg),*), vec![]);
	}};
	( $key:ident $v:expr, $( $arg:tt ), * ) => {{
		logger::log_macro(logger::LoggerLevel::$key, format!($($arg),*), $v);
	}};
}
