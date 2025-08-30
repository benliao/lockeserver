

//! # lockserver
//!
//! A distributed lock server for coordinating access to shared resources across distributed systems.
//!
//! ## Features
//! - Simple API for acquiring and releasing locks
//! - TCP and HTTP API
//! - Client library with ergonomic macros (`lock_scope!`)
//! - Blocking and non-blocking lock acquisition
//!
//! ## Example
//! ```rust
//! use lockserver::{LockManager, LockserverClient, lock_scope};
//! let manager = LockManager::new();
//! let client = LockserverClient::new("127.0.0.1:4000", "worker1");
//! lock_scope!(&client, "resource", {
//!     // critical section
//! });
//! ```

mod lock_manager;

pub mod client;
pub use client::{LockserverClient, LockGuard};

pub use crate::lock_manager::{LockManager, LockError};
