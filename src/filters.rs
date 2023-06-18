use crate::LogLevel;

/// Generate a filter closure which return true if
/// the logging level is between a minimum and a maximum 
pub fn get_level_filter(min: LogLevel, max: LogLevel) -> 
impl Fn(LogLevel, &str) -> bool 
{
    move |level: LogLevel, _: &str| {
        level >= min && level <= max
    }
}
