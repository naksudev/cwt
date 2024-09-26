use simplelog::{Config, LevelFilter, SimpleLogger};

pub fn init_logging() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
}
