#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("");
        println!("INFO: {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        println!("");
        println!("WARNING: {}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        println!("");
        eprintln!("ERROR: {}", format_args!($($arg)*));
    };
}

pub use crate::log_error;
pub use crate::log_info;
pub use crate::log_warning;
