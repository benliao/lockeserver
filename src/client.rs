//! lockserver_client: A client library for interacting with a lockserver instance.
//!
//! Provides macros for easy lock usage, similar to a local mutex guard.
//!
//! # Example
//! ```rust
//! use lockserver::{LockserverClient, lock_scope};
//! let client = LockserverClient::new("127.0.0.1:4000", "worker1");
//! lock_scope!(&client, "resource", {
//!     // critical section
//! });
//! ```

use std::time::Duration;
use std::io::{self, Write, BufRead, BufReader};
use std::net::TcpStream;

/// A client for connecting to a lockserver instance.
///
/// Use this to acquire and release distributed locks.
pub struct LockserverClient {
    addr: String,
    owner: String,
}

/// Lock acquisition mode: blocking or non-blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockMode {
    /// Wait until the lock is acquired.
    Blocking,
    /// Return immediately if the lock is held by another worker.
    NonBlocking,
}

impl LockserverClient {
    /// Create a new client for the given server address and owner ID.
    pub fn new(addr: impl Into<String>, owner: impl Into<String>) -> Self {
        Self { addr: addr.into(), owner: owner.into() }
    }

    /// Try to acquire a lock. If mode is Blocking, will retry every 200ms until success.
    /// Acquire a lock on a resource. Blocks until the lock is acquired.
    pub fn acquire(&self, resource: &str) -> io::Result<()> {
        self.acquire_with_mode(resource, LockMode::Blocking)
    }

    /// Acquire a lock on a resource, with blocking or non-blocking mode.
    ///
    /// Returns an error if the lock cannot be acquired in non-blocking mode.
    pub fn acquire_with_mode(&self, resource: &str, mode: LockMode) -> io::Result<()> {
        loop {
            let mut stream = TcpStream::connect(&self.addr)?;
            let cmd = format!("ACQUIRE {} {}\n", resource, self.owner);
            stream.write_all(cmd.as_bytes())?;
            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            reader.read_line(&mut response)?;
            if response.trim() == "OK" {
                return Ok(());
            } else if mode == LockMode::NonBlocking {
                return Err(io::Error::new(io::ErrorKind::WouldBlock, response));
            } else {
                std::thread::sleep(Duration::from_millis(200));
            }
        }
    }

    /// Release a lock on a resource.
    pub fn release(&self, resource: &str) -> io::Result<()> {
        let mut stream = TcpStream::connect(&self.addr)?;
        let cmd = format!("RELEASE {} {}\n", resource, self.owner);
        stream.write_all(cmd.as_bytes())?;
        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader.read_line(&mut response)?;
        if response.trim() == "OK" {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, response))
        }
    }
}

/// Macro to acquire a distributed lock for a code block.
/// Usage:
///
/// See `tests/lock_scope_macro.rs` for a working example as a regular test.
/// Macro to acquire a distributed lock for a code block.
///
/// # Examples
///
/// Blocking (default):
///
/// See `tests/lock_scope_macro.rs` for a working example as a regular test.
///
/// Non-blocking:
/// ```
/// use lockserver::{lock_scope, LockserverClient};
/// use lockserver::client::LockMode;
/// let client = LockserverClient::new("127.0.0.1:4000", "worker1");
/// lock_scope!(&client, "resource", non_blocking, {
///     // critical section
/// });
/// ```
#[macro_export]
macro_rules! lock_scope {
    // Default: blocking
    ($client:expr, $resource:expr, $block:block) => {{
        $client.acquire($resource).expect("Failed to acquire lock");
        let _guard = $crate::LockGuard::new($client, $resource);
        let result = (|| $block)();
        result
    }};
    // Non-blocking mode
    ($client:expr, $resource:expr, non_blocking, $block:block) => {{
        $client.acquire_with_mode($resource, lockserver::client::LockMode::NonBlocking).expect("Failed to acquire lock (non-blocking)");
        let _guard = $crate::LockGuard::new($client, $resource);
        let result = (|| $block)();
        result
    }};
}

/// RAII guard for releasing a distributed lock when dropped.
pub struct LockGuard<'a> {
    client: &'a LockserverClient,
    resource: &'a str,
}

impl<'a> LockGuard<'a> {
    /// Create a new lock guard. Usually not called directly; use the macro.
    pub fn new(client: &'a LockserverClient, resource: &'a str) -> Self {
        Self { client, resource }
    }
}

impl<'a> Drop for LockGuard<'a> {
    /// Releases the lock when the guard is dropped.
    fn drop(&mut self) {
        let _ = self.client.release(self.resource);
    }
}
