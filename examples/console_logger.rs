use log::*;
use acta::*;

fn main() {
    let stdout = handlers::StdoutHandler::new(
        formatters::time_lvl_target("%H:%M:%S"),
        filters::level_range(LevelFilter::Info, LevelFilter::Warn),    
    );

    init_logger!(stdout).unwrap();

    trace!("test {}", 5);
    debug!("test {}", 4);
    info!("test {}", 3);
    warn!("test {}", 2);
    error!("test {}", 1);
}
