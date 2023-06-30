use log::Metadata;

use crate::{TimedRecord, RecordFormatter};

/// handler trait
/// Required implementation: log, filter, flush
pub trait LogHandler: Sync + Send {
    /// Return true if the log meet the criteria
    /// to be logged by the handler
    fn filter(&self, metadata: &Metadata) -> bool;

    /// Handle a log object
    fn log(&self, record: &TimedRecord);

    /// Flush the cached data
    fn flush(&self);
}

/*
*
*   Stdout Handler definition and implementations
*
*/

/// Print the logs to stdout
pub struct StdoutHandler<F, G>
where
    G: Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result + Send + Sync,
    F: Fn(&Metadata) -> bool + Send + Sync,
{
    /// Fn object to format the log record
    /// Return a string and takes time, level and target as arguments
    formatter: RecordFormatter<G>,

    /// Fn object which return true only if the log need to be handled by the handler
    /// Take log level and target as arguments
    filter: F,
}

// Implement constructor and public methods for StdoutHandler
impl<F, G> StdoutHandler<F, G>
where
    G: Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result + Send + Sync,
    F: Fn(&Metadata) -> bool + Send + Sync,
{
    /// Create a new instance of the Stdout handler
    pub fn new(formatter: G, filter: F) -> Self {
        Self {
            formatter: RecordFormatter::new(formatter), 
            filter, 
        }
    }
}

// Implement the LogHandler trait for StdoutHandler
impl<F, G> LogHandler for StdoutHandler<F, G>
where
    G: Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result + Send + Sync,
    F: Fn(&Metadata) -> bool + Send + Sync,
{
    /// Return true if the log meet the criteria
    /// to be logged by the handler
    fn filter(&self, metadata: &Metadata) -> bool {
        (self.filter)(metadata)
    }

    /// Handle a log object
    fn log(&self, record: &TimedRecord) {
        println!("{}", self.formatter.format(record));
    }

    /// Empty flush implementation
    fn flush(&self) {}
}

/*
*
*   Stderr Handler definition and implementations
*
*/

/// Print the logs to stderr
pub struct StderrHandler<F, G>
where
    G: Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result + Send + Sync,
    F: Fn(&Metadata) -> bool + Send + Sync,
{
    /// Fn object to format the log record
    /// Return a string and takes time, level and target as arguments
    formatter: RecordFormatter<G>,

    /// Fn object which return true only if the log need to be handled by the handler
    /// Take log level and target as arguments
    filter: F,
}

// Implement constructor and public methods for StderrHandler
impl<F, G> StderrHandler<F, G>
where
    G: Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result + Send + Sync,
    F: Fn(&Metadata) -> bool + Send + Sync,
{
    /// Create a new instance of the Stdout handler
    pub fn new(formatter: G, filter: F) -> Self {
        Self {
            formatter: RecordFormatter::new(formatter), 
            filter, 
        }
    }
}

// Implement the LogHandler trait for StderrHandler
impl<F, G> LogHandler for StderrHandler<F, G>
where
    G: Fn(&mut std::fmt::Formatter<'_>, &TimedRecord) -> std::fmt::Result + Send + Sync,
    F: Fn(&Metadata) -> bool + Send + Sync,
{
    /// Return true if the log meet the criteria
    /// to be logged by the handler
    fn filter(&self, metadata: &Metadata) -> bool {
        (self.filter)(metadata)
    }

    /// Handle a log object
    fn log(&self, record: &TimedRecord) {
        eprintln!("{}", self.formatter.format(record));
    }

    /// Empty flush implementation
    fn flush(&self) {}
}
