use chrono::Local;
use colored::Colorize;

#[derive(Debug)]
pub enum LogLevel {
    Log,
    Debug,
    Info,
    Warning,
    Error,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Log => String::from("LOG    "),
            LogLevel::Debug => String::from("DEBUG  "),
            LogLevel::Info => String::from("INFO   "),
            LogLevel::Warning => String::from("WARNING"),
            LogLevel::Error => String::from("ERROR  "),
        }
    }
}

// Make the function public so macros can access it when the library is used
pub fn build_log_str(level: LogLevel, message: &str) -> String {
    let now = Local::now();
    let log_level = level.to_string();
    let log_level_styled = match level {
        LogLevel::Log => log_level.green().bold(),
        LogLevel::Debug => log_level.blue().bold(),
        LogLevel::Info => log_level.cyan().bold(),
        LogLevel::Warning => log_level.yellow().bold(),
        LogLevel::Error => log_level.red().bold(),
    };

    format!(
        "{} [{}] {}",
        log_level_styled,
        now.format("%Y-%m-%d %H:%M:%S").to_string().dimmed(),
        message
    )
}

// Macro definitions with public visibility for use outside the library
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!("{}", $crate::build_log_str($crate::LogLevel::Log, &format!($($arg)*)));
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        println!("{}", $crate::build_log_str($crate::LogLevel::Debug, &format!($($arg)*)));
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        println!("{}", $crate::build_log_str($crate::LogLevel::Info, &format!($($arg)*)));
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        println!("{}", $crate::build_log_str($crate::LogLevel::Warning, &format!($($arg)*)));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        println!("{}", $crate::build_log_str($crate::LogLevel::Error, &format!($($arg)*)));
    };
}
