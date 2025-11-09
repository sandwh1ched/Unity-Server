/* main.rs -- primary logic
 * All code in Unity is licensed under the BSD two-clause license;
 * see `LICENSE` in the project root.
 */

use libunity::{LogLevel, log};

/// The entrypoint to the control flow for the Unity Server core.
fn main() {
    // Print splash text
    println!(
        "\x1b[1;34m<< Unity Server v{} >>\x1b[0m",
        env!("CARGO_PKG_VERSION")
    );

    // Run preliminary tests
    log("Running preliminary tests.", LogLevel::Info, None);

    // Spin up Unity apps
    // Enter primary control flow loop
    loop {}
}
