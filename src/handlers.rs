use crate::{Log, LogLevel};

use chrono::{DateTime, Local};
use colored::Colorize;


pub trait LogHandler: Sync + Send {
    /// Handle a log object
    fn log(&self, log: &Log);

    /// Return true if the log meet the criteria 
    /// to be logged by the handler
    fn filter(&self, log: &Log) -> bool;
}

/// Print the logs to stdout
pub struct StdoutHandler<F, G>
where
    G: Fn(&DateTime<Local>, LogLevel, &str) -> String + Send + Sync,
    F: Fn(LogLevel, &str) -> bool + Send + Sync,
{
    /// Fn object to format the log record
    /// Return a string and takes time, level and target as arguments
    header_formatter: G,

    /// Fn object which return true only if the log need to be handled by the handler
    /// Take log level and target as arguments
    filter: F,

    /// If set to true give a color based on the log level to the record
    color: bool,
}

/// Print the logs to stderr
pub struct StderrHandler<F, G>
where
    G: Fn(&DateTime<Local>, LogLevel, &str) -> String + Send + Sync,
    F: Fn(LogLevel, &str) -> bool + Send + Sync,
{
    /// Fn object to format the log record
    /// Return a string and takes time, level and target as arguments
    header_formatter: G,

    /// Fn object which return true only if the log need to be handled by the handler
    /// Take log level and target as arguments
    filter: F,

    /// If set to true give a color based on the log level to the record
    color: bool,
}

// Implement constructor and public methods for StdoutHandler
impl<F, G> StdoutHandler<F, G>
where
    G: Fn(&DateTime<Local>, LogLevel, &str) -> String + Send + Sync,
    F: Fn(LogLevel, &str) -> bool + Send + Sync,
{    
    /// Create a new instance of the Stdout handler
    /// By default colors are enables
    /// use the console_colors function to disable them
    pub fn new(header_formatter: G, filter: F) -> Self {
        Self {
            header_formatter,
            filter,

            color: true,
        }
    }

    /// Enable or disable colored output in stdout
    pub fn console_colors(&mut self, colors_enable: bool) {
        self.color = colors_enable;
    }
}

// Implement constructor and public methods for StderrHandler
impl<F, G> StderrHandler<F, G>
where
    G: Fn(&DateTime<Local>, LogLevel, &str) -> String + Send + Sync,
    F: Fn(LogLevel, &str) -> bool + Send + Sync,
{    
    /// Create a new instance of the Stdout handler
    /// By default colors are enables
    /// use the console_colors function to disable them
    pub fn new(header_formatter: G, filter: F) -> Self {
        Self {
            header_formatter,
            filter,

            color: true,
        }
    }

    /// Enable or disable colored output in stderr
    pub fn console_colors(&mut self, colors_enable: bool) {
        self.color = colors_enable;
    }
}

// Implement the LogHandler trait for StdoutHandler
impl<F, G> LogHandler for StdoutHandler<F, G>
where
    G: Fn(&DateTime<Local>, LogLevel, &str) -> String + Send + Sync,
    F: Fn(LogLevel, &str) -> bool + Send + Sync,
{
    /// Return true if the log meet the criteria 
    /// to be logged by the handler
    fn filter(&self, log: &Log) -> bool {
        (self.filter)(log.level, log.source)
    }

    /// Handle a log object
    fn log(&self, log: &Log) {
        let log_record = (self.header_formatter)(
            &log.time,
            log.level,
            log.source,
        );

        if self.color {
            println!("{}{}", log_record.color(log.level.get_color()), log.message);
        } else {
            println!("{}{}", log_record, log.message);
        }
    }
}

// Implement the LogHandler trait for StderrHandler
impl<F, G> LogHandler for StderrHandler<F, G>
where
    G: Fn(&DateTime<Local>, LogLevel, &str) -> String + Send + Sync,
    F: Fn(LogLevel, &str) -> bool + Send + Sync,
{
    /// Return true if the log meet the criteria 
    /// to be logged by the handler
    fn filter(&self, log: &Log) -> bool {
        (self.filter)(log.level, log.source)
    }

    /// Handle a log object
    fn log(&self, log: &Log) {
        let log_record = (self.header_formatter)(
            &log.time,
            log.level,
            log.source,
        );

        if self.color {
            println!("{}{}", log_record.color(log.level.get_color()), log.message);
        } else {
            println!("{}{}", log_record, log.message);
        }
    }
}
