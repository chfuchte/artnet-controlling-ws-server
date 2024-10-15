use colored::Colorize;
use chrono::Local;

enum LogLevel {
    Log,
    Debug,
    Info,
    Warning,
    Error,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Log => String::from("LOG"),
            LogLevel::Debug => String::from("DEBUG"),
            LogLevel::Info => String::from("INFO"),
            LogLevel::Warning => String::from("WARNING"),
            LogLevel::Error => String::from("ERROR"),
        }
    }
}

fn build_log_str(level: LogLevel, message: &str) -> String {
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

pub fn log(message: &str) {
    println!("{}", build_log_str(LogLevel::Log, message))
}

pub fn debug(message: &str) {
    println!("{}", build_log_str(LogLevel::Debug, message))
}

pub fn info(message: &str) {
    println!("{}", build_log_str(LogLevel::Info, message))
}

pub fn warning(message: &str) {
    println!("{}", build_log_str(LogLevel::Warning, message))
}

pub fn error(message: &str) {
    println!("{}", build_log_str(LogLevel::Error, message))
}
