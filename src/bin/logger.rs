use log::{LevelFilter, debug, error, info, trace, warn};
use rustbook::logs::init_logger;

fn main() {
    init_logger();
    info!("this is {}", LevelFilter::Info);
    error!("this is {}", LevelFilter::Error);
    debug!("this is {}", LevelFilter::Debug);
    warn!("this is {}", LevelFilter::Warn);
    trace!("this is {}", LevelFilter::Trace);
}