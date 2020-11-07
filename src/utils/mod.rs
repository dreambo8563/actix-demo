pub mod logger;
pub mod settings;
pub mod wrapper;
use lazy_static::lazy_static;

lazy_static! {
    /// singleton -  Env config
    pub static ref ENV: settings::Settings = settings::Settings::new().unwrap();
}

lazy_static! {
    // singleton - logger instance
    pub static ref LOGGING: slog::Logger = logger::Logging::new();
}
