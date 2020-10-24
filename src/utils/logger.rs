use crate::utils::ENV;
use slog::LevelFilter;

use slog::{o, Drain, FnValue, Level, Logger};
use std::fs::OpenOptions;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Logging {
    pub logger: Logger,
}

impl Logging {
    pub fn new() -> Self {
        debug!("{:?}", ENV.database);
        let logconfig = &ENV.logconfig;
        let logfile = &logconfig.path;
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(logfile)
            .unwrap();

        let filter_level = &logconfig.level;
        let filter_level = filter_level
            .parse::<Level>()
            .expect("Invalid log level filter");

        let applogger = Logger::root(
            Mutex::new(LevelFilter::new(slog_bunyan::default(file), filter_level)).fuse(),
            o!("location" => FnValue(move |info| {
                format!("{}:{} {}", info.file(), info.line(), info.module())
                })
            ),
        );

        Logging { logger: applogger }
    }
}
