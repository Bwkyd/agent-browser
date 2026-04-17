//! Minimal example: launch Chrome for Testing via the `agent_browser` lib API.
//!
//! Run:
//! ```bash
//! cargo run --example launch_and_fetch
//! ```
//!
//! Expected output: a line like `launched chrome pid=12345 ws_url=ws://…`
//! followed by the process gracefully terminating.
//!
//! Prerequisite: Chrome for Testing already installed in the user cache
//! (run the `agent-browser` binary once, or use `install::run_install`).

use agent_browser::native::cdp::chrome::{launch_chrome, LaunchOptions};

fn main() -> Result<(), String> {
    let opts = LaunchOptions {
        headless: true,
        ..Default::default()
    };

    let mut chrome = launch_chrome(&opts)?;
    println!(
        "launched chrome pid={} ws_url={}",
        chrome.id(),
        chrome.ws_url
    );

    // Give Chrome a moment to be fully ready, then shut down cleanly.
    std::thread::sleep(std::time::Duration::from_millis(500));
    chrome.kill();
    println!("shutdown ok");

    Ok(())
}
