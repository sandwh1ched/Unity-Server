/* configuration.rs -- support for configuration files
 * All code in Unity is licensed under the BSD two-clause license;
 * see `LICENSE` in the project root.
 */

use serde::Deserialize;
use std::{fs::read_to_string, io::Error as IoError, path::PathBuf};
use thiserror::Error;
use toml::de as deserialize;

/// The path that the configuration file is at.
pub static CONFIG_PATH: &str = "/srv/unity/config.toml";
/// Default apps path.
pub static DEFAULT_APPS_PATH: &str = "/srv/unity/apps";

/// A configuration.
#[derive(Deserialize)]
pub struct Configuration {
    /// If local time should be used in logs. Generally only recommended if
    /// you don't administrate multiple servers across timezones and you hate
    /// UTC. (UTC will be converted into local time in the frontend log viewer.)
    pub use_local_time_in_logs: bool,
    /// The directory Unity apps will be stored in.
    pub apps_directory_path: PathBuf,
}

impl Configuration {
    /// Generates a default configuration.
    ///
    /// # Returns
    /// The default configuration, which is below.
    /// ```toml
    /// use_local_time_in_logs = false
    /// apps_directory_path = "/srv/unity/apps"
    /// ```
    pub fn new() -> Configuration {
        Configuration {
            use_local_time_in_logs: false,
            apps_directory_path: PathBuf::from(DEFAULT_APPS_PATH),
        }
    }

    /// Loads the configuration from the file.
    ///
    /// # Returns
    /// `Result<Configuration, Error>`, `Ok(Configuration)` on success, and `Err(Error)`.
    ///
    /// # Example
    /// ```rust
    /// // /etc/unity/config.toml:
    /// // use_local_time_in_logs = true
    /// let loaded_configuration = load_configuration_from_default_path();
    /// let new_configuration = Configuration::new();
    /// new_configuration.use_local_time_in_logs = true;
    /// assert_eq!(loaded_configuration, new_configuration);
    /// ```
    pub fn load() -> Result<Configuration, Error> {
        Ok(deserialize::from_str(
            read_to_string(CONFIG_PATH)?.as_str(),
        )?)
    }
}

/// An enumeration of possible configuration-related errors.
#[derive(Error, Debug)]
pub enum Error {
    /// I/O (read/write) error.
    #[error("I/O (read/write) error")]
    ReadWriteError(#[from] IoError),
    /// Configuration file deserialization error.
    #[error("configuration file deserialization error")]
    DeserializationError(#[from] deserialize::Error),
}
