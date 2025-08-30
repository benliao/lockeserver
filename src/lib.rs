
//! # lockserver
//!
//! A distributed lock server for coordinating access to shared resources.
//!
//! ## Features
//! - Simple API for acquiring and releasing locks
//! - TCP and HTTP API
//! - Client library with ergonomic macros
//!
//! ## Example
//! ```rust
//! use lockserver::{LockManager, LockserverClient, lock_scope};
//! // ...
//! ```

mod lock_manager;

pub mod client;
pub use client::{LockserverClient, LockGuard};

pub use crate::lock_manager::{LockManager, LockError};
