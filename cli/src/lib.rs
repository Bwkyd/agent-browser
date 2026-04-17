//! agent-browser as a library.
//!
//! Exposes the browser-automation primitives (CDP protocol, Chrome launch,
//! cookie bridging, Chrome for Testing install) so that external Rust crates
//! can reuse them without shelling out to the `agent-browser` binary.
//!
//! # Stability
//!
//! `0.x` — the API may evolve between minor versions. For build reproducibility,
//! pin by `rev`:
//!
//! ```toml
//! agent_browser = { git = "https://github.com/vercel-labs/agent-browser", rev = "..." }
//! ```
//!
//! # Scope
//!
//! Intentionally minimal. Re-exports:
//! - [`install`] — Chrome for Testing download & bootstrap
//! - [`native`] — CDP client, browser launch, cookie / session handling
//!
//! CLI-only modules (`chat`, `commands`, `connection`, `doctor`, `flags`,
//! `output`, `skills`, `upgrade`, `validation`) are **not** re-exported —
//! they are implementation details of the binary.
//!
//! # Example
//!
//! See `examples/launch_and_fetch.rs` for a minimal CDP launch.

pub mod install;
pub mod native;

// -----------------------------------------------------------------------------
// Internal plumbing (not part of the public API).
// These modules are `pub` only because `native::actions` / `native::stream`
// depend on them via `crate::connection::…` / `crate::commands::…`. They are
// marked `#[doc(hidden)]` and should not be used by external crates.
// -----------------------------------------------------------------------------
#[doc(hidden)]
pub mod color;
#[doc(hidden)]
pub mod commands;
#[doc(hidden)]
pub mod connection;
#[doc(hidden)]
pub mod flags;
#[doc(hidden)]
pub mod validation;

#[cfg(test)]
#[doc(hidden)]
pub mod test_utils;
