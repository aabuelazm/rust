#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}
pub fn info(message: &str) -> String {
    "[INFO]: ".to_owned() + message
}
pub fn warn(message: &str) -> String {
    "[WARNING]: ".to_owned() + message
}
pub fn error(message: &str) -> String {
    "[ERROR]: ".to_owned() + message
}
