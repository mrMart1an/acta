pub mod handlers;
pub mod formatters;
pub mod filters;
pub mod macros;

use colored::Color;
use handlers::LogHandler;
use core::fmt;
use std::sync::RwLock;

use chrono::prelude::*;
use lazy_static::lazy_static;
use std::fmt::Display;

// Lazy initialization of the LOGGER static variable
// with an empty handlers vector
lazy_static! {
    static ref LOGGER: Logger = Logger {
        handlers: RwLock::new(vec![])
    };
}

// Compile time log level const
pub const COMPILE_LOG_LEVEL: Option<LogLevel> = compile_log_level();
   
pub struct Logger {
    handlers: RwLock<Vec<Box<dyn LogHandler>>>,
}

/// Define a struct to represent a log entry
pub struct Log<'a> {
    /// The time of creation of the log
    pub time: DateTime<Local>,

    /// The log level assigned to the log
    pub level: LogLevel,
    /// The formatted logging message
    pub message: fmt::Arguments<'a>,
    /// The source of the logging message
    pub source: &'a str,
}

// Implement public methods of the Logger struct 
impl Logger {
    /// Private api function: use the log_add_handlers macro instead
    ///
    /// Add a log handler to the global logger
    /// Each log will be processed by all the handlers 
    pub fn __private_add_handler(handler: Box<dyn LogHandler>) {
        LOGGER.handlers.write().unwrap().push(handler);
    }

    /// Private api function: use the log macro instead
    ///
    /// Generate a new log and process it
    pub fn __private_log(level: LogLevel, source: &str, fmt_args: fmt::Arguments) {
        let log = Log {
            message: fmt_args,
            source,

            level,
            time: Local::now(),
        };

        // Run the log through all the handler
        for handler in LOGGER.handlers.read().unwrap().iter() {
            if handler.filter(&log) {
                handler.log(&log);
            }
        }
    }
}

/// Emum to represent the log level 
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

// Implement the Display trait for the log level Enum 
impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "Trace"),
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Info => write!(f, "Info "),
            LogLevel::Warning => write!(f, "Warn "),
            LogLevel::Error => write!(f, "Error"),
            LogLevel::Fatal => write!(f, "Fatal"),
        }
    }
}

// Implement public methods of the log level Enum
impl LogLevel {
    /// Return the colored library color for the corresponding log level
    pub fn get_color(&self) -> Color {
        match self {
            LogLevel::Trace => Color::BrightGreen,
            LogLevel::Debug => Color::Green,
            LogLevel::Info => Color::Blue,
            LogLevel::Warning => Color::Yellow,
            LogLevel::Error => Color::BrightRed,
            LogLevel::Fatal => Color::Red,
        }
    }
}
 
// Obtain the compile time log level for debug builds
#[allow(unreachable_code)]
#[cfg(debug_assertions)]
const fn compile_log_level() -> Option<LogLevel> {
    #[cfg(feature = "debug_log_lvl_trace")]
    return Some(LogLevel::Trace);

    #[cfg(feature = "debug_log_lvl_debug")]
    return Some(LogLevel::Debug);

    #[cfg(feature = "debug_log_lvl_info")]
    return Some(LogLevel::Info);

    #[cfg(feature = "debug_log_lvl_warn")]
    return Some(LogLevel::Warning);

    #[cfg(feature = "debug_log_lvl_error")]
    return Some(LogLevel::Error);

    #[cfg(feature = "debug_log_lvl_fatal")]
    return Some(LogLevel::Fatal);

    #[cfg(feature = "debug_log_lvl_off")]
    return None;

    // If no feature is specified default to None to disable 
    // logging completely 
    None
}

// Obtain the compile time log level for release builds
#[allow(unreachable_code)]
#[cfg(not(debug_assertions))]
const fn compile_log_level() -> Option<LogLevel> {
    #[cfg(feature = "release_log_lvl_trace")]
    return Some(LogLevel::Trace);

    #[cfg(feature = "release_log_lvl_debug")]
    return Some(LogLevel::Debug);

    #[cfg(feature = "release_log_lvl_info")]
    return Some(LogLevel::Info);

    #[cfg(feature = "release_log_lvl_warn")]
    return Some(LogLevel::Warning);

    #[cfg(feature = "release_log_lvl_error")]
    return Some(LogLevel::Error);

    #[cfg(feature = "release_log_lvl_fatal")]
    return Some(LogLevel::Fatal);
    
    #[cfg(feature = "release_log_lvl_off")]
    return None;

    // If no feature is specified default to None to disable 
    // logging completely 
    None
}
