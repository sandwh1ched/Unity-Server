/* main.rs -- primary logic
 * All code in Unity is licensed under the BSD two-clause license;
 * see `LICENSE` in the project root.
 */

/// Support for configuration files.
pub mod configuration;
/// Unit and integration tests.
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
        app.unwrap_or("Unity Core"),
        level,
        chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ"),
        message
    );
}

/// The entrypoint to the control flow for the Unity Server core.
fn main() {
    println!("Unity Server v{}", env!("CARGO_PKG_VERSION"));

    // Spin up Unity apps
    // Enter primary control flow loop
}
