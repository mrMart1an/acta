use log::{LevelFilter, Metadata};

/// Generate an empty filter which always return true
pub fn empty() ->
impl Fn(&Metadata) -> bool 
{
    |_: &Metadata| true
}

/// Generate a filter closure which return true if
/// the log level is equal or below the given min value
pub fn level_min(min: LevelFilter) ->
impl Fn(&Metadata) -> bool 
{
    move |metadata: &Metadata| {
        metadata.level() >= min
    }
}

/// Generate a filter closure which return true if
/// the log level is equal or above the given max value
pub fn level_max(max: LevelFilter) ->
impl Fn(&Metadata) -> bool 
{
    move |metadata: &Metadata| {
        metadata.level() <= max
    }
}

/// Generate a filter closure which return true if
/// the log level is within the given min and max values
pub fn level_range(max: LevelFilter, min: LevelFilter) -> 
impl Fn(&Metadata) -> bool 
{
    move |metadata: &Metadata| {
        metadata.level() <= max && metadata.level() >= min
    }
}

/// Generate a filter closure which return true if
/// the log came from a specific target
pub fn target(target: &'static str) -> 
impl Fn(&Metadata) -> bool 
{
    move |metadata: &Metadata| {
        metadata.target() == target 
    }
}
