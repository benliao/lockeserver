use dotenvy::dotenv;
use std::env;
impl LockserverClient {
    /// Create a new client, loading address and owner from environment variables or .env if not provided.
    ///
    /// - `LOCKSERVER_ADDR` (default: "127.0.0.1:8080")
    /// - `LOCKSERVER_OWNER` (default: "default_owner")
    pub fn new_with_env(addr: Option<impl Into<String>>, owner: Option<impl Into<String>>) -> Self {
        let _ = dotenv();
        let addr = addr
            .map(|a| a.into())
            .or_else(|| env::var("LOCKSERVER_ADDR").ok())
            .unwrap_or_else(|| "127.0.0.1:8080".to_string());
        let owner = owner
            .map(|o| o.into())
            .or_else(|| env::var("LOCKSERVER_OWNER").ok())
            .unwrap_or_else(|| "default_owner".to_string());
        Self { addr, owner }
    }
}
use std::io;
use reqwest::blocking::Client as HttpClient;
use reqwest::StatusCode;
use serde::Serialize;
/// # lockserver_client
///
/// A Rust client library for interacting with a lockserver HTTP instance.
///
/// Provides macros for easy distributed lock usage, similar to a local mutex guard.
///
/// ## Configuration
///
/// The client can load the server address and owner from environment variables or a `.env` file:
///
/// - `LOCKSERVER_ADDR` (default: `127.0.0.1:8080`)
/// - `LOCKSERVER_OWNER` (default: `default_owner`)
///
/// ## Example
/// ```rust
/// use lockserver::{LockserverClient, lock_scope};
/// // Loads from env/.env if None
/// let client = LockserverClient::new_with_env(None::<String>, None::<String>);
/// lock_scope!(&client, "resource", {
///     // critical section
/// });
/// ```

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
        Self {
            addr: addr.into(),
            owner: owner.into(),
        }
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
        #[derive(Serialize)]
        struct LockRequest<'a> {
            resource: &'a str,
            owner: &'a str,
        }
        let client = HttpClient::new();
        let url = format!("http://{}/acquire", self.addr);
        let req = LockRequest { resource, owner: &self.owner };
        loop {
            let resp = client.post(&url).json(&req).send();
            match resp {
                Ok(r) if r.status() == StatusCode::OK => return Ok(()),
                Ok(r) if r.status() == StatusCode::CONFLICT => {
                    if mode == LockMode::NonBlocking {
                        return Err(io::Error::new(io::ErrorKind::WouldBlock, "Resource is locked"));
                    } else {
                        std::thread::sleep(std::time::Duration::from_millis(200));
                    }
                }
                Ok(r) => {
                    return Err(io::Error::other(format!("HTTP error: {}", r.status())));
                }
                Err(e) => {
                    return Err(io::Error::other(format!("Request error: {}", e)));
                }
            }
        }
    }

    /// Release a lock on a resource.
    pub fn release(&self, resource: &str) -> io::Result<()> {
        #[derive(Serialize)]
        struct LockRequest<'a> {
            resource: &'a str,
            owner: &'a str,
        }
        let client = HttpClient::new();
        let url = format!("http://{}/release", self.addr);
        let req = LockRequest { resource, owner: &self.owner };
        let resp = client.post(&url).json(&req).send();
        match resp {
            Ok(r) if r.status() == StatusCode::OK => Ok(()),
            Ok(r) => Err(io::Error::other(format!("HTTP error: {}", r.status()))),
            Err(e) => Err(io::Error::other(format!("Request error: {}", e))),
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
/// let client = LockserverClient::new("127.0.0.1:8080", "worker1");
/// lock_scope!(&client, "resource_non_blocking", non_blocking, {
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
        $client
            .acquire_with_mode($resource, lockserver::client::LockMode::NonBlocking)
            .expect("Failed to acquire lock (non-blocking)");
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
