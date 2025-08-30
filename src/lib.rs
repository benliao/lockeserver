//! # lockserver
//!
//! A distributed lock server for coordinating access to shared resources across distributed systems.
//!
//! ## Features
//! - Simple API for acquiring and releasing locks
//! - HTTP API only (no TCP service)
//! - Client library with ergonomic macros (`lock_scope!`)
//! - Blocking and non-blocking lock acquisition
//!
//! ## Example
//! ```rust
//! use lockserver::{LockManager, LockserverClient, lock_scope};
//! let manager = LockManager::new();
//! let client = LockserverClient::new_with_env(None::<String>, None::<String>);
//! lock_scope!(&client, "resource", {
//!     // critical section
//! });
//! ```

mod lock_manager;

pub mod client;
pub use client::{LockGuard, LockserverClient};

pub use crate::lock_manager::{LockError, LockManager};
