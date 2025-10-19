/* tests/configuration.rs -- unit and integration tests for the configuration module
 * All code in Unity is licensed under the BSD two-clause license;
 * see `LICENSE` in the project root.
 */

#[allow(unused_imports)]
use crate::*;

#[test]
pub fn read_config() {
    configuration::Configuration::new();
}
