use crate::TimedRecord;

/// Obtain a default formatter which include the source field
///
/// Format: [%time %log_level] %target => %message 
pub fn time_lvl_target(time_formatter: &'static str) -> 
impl Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result 
{
    move |f: &mut std::fmt::Formatter<'_>, record: &TimedRecord| {
        write!(f,
            "[{} {}] {} => {}",
            record.time().format(time_formatter),
            record.level(),
            record.target(),
            record.args()
        )
    }
}

/// Obtain a default formatter which include the source field
///
/// Format: [%time %log_level] => %message
pub fn time_lvl(time_formatter: &'static str) -> 
impl Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result 
{
    move |f: &mut std::fmt::Formatter<'_>, record: &TimedRecord| {
        write!(f,
            "[{} {}] => {}",
            record.time().format(time_formatter),
            record.level(),
            record.args()
        )
    }
}
