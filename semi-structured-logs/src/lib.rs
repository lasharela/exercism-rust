// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// #![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    return format!("[{}]: {}", get_log_prefix(level), message);
}
pub fn info(message: &str) -> String {
    return format!("[{}]: {}", get_log_prefix(LogLevel::Info), message);
}
pub fn warn(message: &str) -> String {
    return format!("[{}]: {}", get_log_prefix(LogLevel::Warning), message);
}
pub fn error(message: &str) -> String {
    return format!("[{}]: {}", get_log_prefix(LogLevel::Error), message);
}

fn get_log_prefix(level: LogLevel) -> String {
    let prefix = match level {
        LogLevel::Error => "ERROR",
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Debug => "DEBUG",
    };
    return prefix.to_string();
}
