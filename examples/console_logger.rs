use acta::*;

fn main() {
    let stdout = handlers::StdoutHandler::new(
        formatters::get_fmt_without_source("%Y/%m/%d"), 
        filters::get_level_filter(LogLevel::Trace, LogLevel::Warning),
    );
    let stderr = handlers::StderrHandler::new(
        formatters::get_fmt_with_source("%Y/%m/%d"), 
        filters::get_level_filter(LogLevel::Error, LogLevel::Fatal),
    );

    log_add_handlers!(stdout, stderr);

    trace!("test {}", 1);
    debug!("test {}", 2);
    info!("test {}", 3);
    warn!("test {}", 4);
    error!("test {}", 5);
    fatal!("test {}", 6);

    println!("compile time log level: {:?}", COMPILE_LOG_LEVEL);
}
