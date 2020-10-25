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
        let logconfig = &ENV.logconfig;

        // let logfile = &logconfig.path;
        let filter_level = &logconfig.level;
        let filter_level = filter_level
            .parse::<Level>()
            .expect("Invalid log level filter");

        match &logconfig.path {
            Some(logfile) => {
                let file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(logfile)
                    .unwrap();
                let applogger = Logger::root(
                    Mutex::new(LevelFilter::new(
                        slog_bunyan::with_name("demo App", file).build(),
                        filter_level,
                    ))
                    .fuse(),
                    o!("location" => FnValue(move |info| {
                        format!("{}:{} {}", info.file(), info.line(), info.module())
                        })
                    ),
                );
                return Logging { logger: applogger };
            }
            None => {
                let output = std::io::stdout();
                let applogger = Logger::root(
                    Mutex::new(LevelFilter::new(
                        slog_bunyan::with_name(&"demo App", output).build(),
                        filter_level,
                    ))
                    .fuse(),
                    o!("location" => FnValue(move |info| {
                        format!("{}:{} {}", info.file(), info.line(), info.module())
                        })
                    ),
                );
                Logging { logger: applogger }
            }
        }
    }
}
