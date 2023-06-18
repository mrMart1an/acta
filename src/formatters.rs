use chrono::{DateTime, Local};

use crate::LogLevel;

/// Obtain a default formatter which include the source field
///
/// Format: [%time %log_level] %source => 
///
/// The log message is appended to the formatter output by the handler
pub fn get_fmt_with_source(time_formatter: &'static str) -> 
impl Fn(&DateTime<Local>, LogLevel, &str) -> String 
{
    move |time: &DateTime<Local>, level: LogLevel, source: &str| {
        format!("[{} {}] {} => ", time.format(time_formatter), level, source)
    }
}

/// Obtain a default formatter which include the source field
///
/// Format: [%time %log_level] => 
///
/// The log message is appended to the formatter output by the handler
pub fn get_fmt_without_source(time_formatter: &'static str) -> 
impl Fn(&DateTime<Local>, LogLevel, &str) -> String 
{
    move |time: &DateTime<Local>, level: LogLevel, _: &str| {
        format!("[{} {}] => ", time.format(time_formatter), level)
    }
}
