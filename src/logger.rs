use std::fmt::Display;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn log<T>(log_level: LogLevel, msg: T)
where
    T: Display,
{
    match log_level {
        LogLevel::Info => {
            println!("INFO: {msg}");
        }
        LogLevel::Warning => {
            println!("WARNING: {msg}");
        }
        LogLevel::Error => {
            eprintln!("ERROR: {msg}");
        }
    }
}
