use crate::utils::ENV;
use slog::LevelFilter;

use slog::{o, Drain, FnValue, Level, Logger};
use std::fs::OpenOptions;
use std::sync::Mutex;

const APP_NAME: &str = "demo App";
pub struct Logging {
    pub logger: Logger,
}

impl Logging {
    pub fn new() -> Logger {
        let logconfig = &ENV.logconfig;

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
                return create_logger(filter_level, file);
            }
            None => {
                let output = std::io::stdout();
                create_logger(filter_level, output)
            }
        }
    }
}

fn create_logger<T: 'static>(lv: slog::Level, dest: T) -> Logger
where
    T: std::io::Write + std::marker::Send,
{
    Logger::root(
        Mutex::new(LevelFilter::new(
            slog_bunyan::with_name(APP_NAME, dest).build(),
            lv,
        ))
        .fuse(),
        o!("location" => FnValue(move |info| {
            format!("{}:{} {}", info.file(), info.line(), info.module())
            })
        ),
    )
}
