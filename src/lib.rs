pub mod filters;
pub mod formatters;
pub mod handlers;
pub mod macros;

use chrono::{DateTime, Local};
use core::fmt::{Arguments, Display};
use handlers::LogHandler;
use log::*;
use std::sync::RwLock;

/*
*
*   Logger definition and implementations
*
*/

pub struct Logger {
    handlers: RwLock<Vec<Box<dyn LogHandler>>>,
}

// Implement initialization methods of the Logger struct
impl Logger {
    /// Private api function: use the log_init macro instead
    ///
    /// Initialized the logger with the given handlers
    pub fn __private_init(handlers: Vec<Box<dyn LogHandler>>) -> Result<(), SetLoggerError> {
        static LOGGER: Logger = Logger {
            handlers: RwLock::new(vec![]),
        };

        *LOGGER
            .handlers
            .write()
            .expect("couldn't get write lock on handlers") = handlers;

        // Initialize logger with static max log level trace
        // The log filtering is done by the handlers
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Trace))
    }
}

// Implement the log crate Logger trait for Logger
impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let timed_record = TimedRecord::new(record);

        self.handlers
            .read()
            .expect("acta: Failed to obtain read lock for handlers")
            .iter()
            .for_each(|handler| {
                if handler.filter(record.metadata()) {
                    handler.log(&timed_record);
                };
            });
    }

    fn flush(&self) {}
}

/*
*
*   TimedRecord definition and implementations
*
*/

/// Store a log record and the time of its creation
/// Implement args, level, target, time and metadata
pub struct TimedRecord<'a> {
    time: DateTime<Local>,
    record: &'a Record<'a>,
}

// Implement methods and constructor for TimedRecord
impl<'a> TimedRecord<'a> {
    /// Generate a new TimedRecord initialized with the current time
    pub fn new(record: &'a Record) -> Self {
        Self {
            time: Local::now(),
            record,
        }
    }

    /// Get a reference to the log initialization time
    pub fn time(&self) -> &DateTime<Local> {
        &self.time
    }

    /// Get a reference to the log message args
    pub fn args(&self) -> &Arguments {
        self.record.args()
    }

    /// Get the log level
    pub fn level(&self) -> Level {
        self.record.level()
    }

    /// Get a reference to the log target
    pub fn target(&self) -> &str {
        self.record.target()
    }

    /// Get a reference to the log metadata
    pub fn metadata(&self) -> &Metadata {
        self.record.metadata()
    }
}

/*
*
*   Formatter definition and implementations
*
*/

// Store the formatter function and generate FormattedRecord
pub struct RecordFormatter<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result + Send + Sync,
{
    formatter_function: F,
}

// implement constructor and methods for RecordFormatter
impl<F> RecordFormatter<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result + Send + Sync,
{
    fn new(formatter_function: F) -> Self {
        RecordFormatter { formatter_function }
    }

    fn format<'a>(&'a self, record: &'a TimedRecord) -> FormattedRecord<'a, &F> {
        FormattedRecord {
            formatter_function: &self.formatter_function,
            record,
        }
    }
}

/*
*
*   Formatted Record definition and implementations
*
*/

// Store a reference to a record and a formatter function
struct FormattedRecord<'a, F>
where
    F: Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result + Send + Sync,
{
    formatter_function: F,
    record: &'a TimedRecord<'a>,
}

// Implement the Display trait for the formatted record
impl<'a, F> Display for FormattedRecord<'a, F>
where
    F: Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result + Send + Sync,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Use the stored formatter function to format the record
        (self.formatter_function)(f, self.record)
    }
}
