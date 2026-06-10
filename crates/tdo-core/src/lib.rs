//! # tdo-core
//!
//! Shared domain logic for the tdo ecosystem (CLI, server, daemon).
//!
//! **No I/O in this crate.** All persistence, networking, and terminal interaction
//! lives in the consuming crates (`tdo-cli`, `tdo-server`, `tdo-daemon`).

pub mod error;
pub mod model;
pub mod render;
pub mod sync;
