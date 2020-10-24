pub mod logger;
pub mod settings;
pub mod wrapper;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ENV: settings::Settings = settings::Settings::new().unwrap();
}

lazy_static! {
    pub static ref LOGGING: logger::Logging = logger::Logging::new();
}
