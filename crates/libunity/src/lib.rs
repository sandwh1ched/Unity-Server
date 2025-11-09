/// TOML configuration file support.
pub mod configuration;
/// Unit tests.
pub mod tests;

/// The level at which a log message is.
#[derive(Debug)]
pub enum LogLevel {
    /// Informational message.
    Info,
    /// Warning message.
    Warning,
    /// Error message.
    Error,
}

/// Informative logging function for Unity Server.
///
/// # Parameters
/// * `message` - The actual message.
/// * `level` - The severity of the message.
/// * `app` - The Unity app issuing the message, if applicable.
///
/// # Example
/// ```rust
/// log("I am very hungry", LogLevel::Info, None);
/// // Result (stdout): [Unity Core] [Info] [2025-10-08T07:42:37Z] I am very hungry
/// ```
///
/// # To-do
/// - Output warnings and errors to `stderr`
/// - Add color?
pub fn log(message: &str, level: LogLevel, app: Option<&str>) {
    println!(
        "[{}] [{:?}] [{}] {}",
        app.unwrap_or("Unity"),
        level,
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ"),
        message
    );
}
