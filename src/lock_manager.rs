//! # lock_manager
//!
//! Lock management for the distributed lock server.
//!
//! This module provides the in-memory lock manager used by the server.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;

/// Errors returned by the lock manager.
#[derive(Debug, thiserror::Error)]
pub enum LockError {
    #[error("Resource is already locked")]
    AlreadyLocked,
    #[error("Resource not found")]
    NotFound,
    #[error("Internal error: {0}")]
    Internal(String),
}

/// In-memory lock manager for distributed locks.

#[derive(Debug)]
struct LockInfo {
    owner: String,
    expire_at: Option<u64>, // unix timestamp in seconds
}

#[derive(Debug, Default)]
pub struct LockManager {
    locks: Arc<Mutex<HashMap<String, LockInfo>>>, // resource -> LockInfo
    timeslots: Arc<Mutex<HashMap<u64, HashSet<String>>>>, // expire_at -> set of resources
}

impl LockManager {
    /// Create a new lock manager.

    pub fn new() -> Self {
        let manager = Self {
            locks: Arc::new(Mutex::new(HashMap::new())),
            timeslots: Arc::new(Mutex::new(HashMap::new())),
        };
        manager.spawn_expiry_worker();
        manager
    }

    /// Try to acquire a lock for a resource and owner, with optional expiration in seconds.
    /// expire_secs: None = no expiration, Some(n) = expire after n seconds
    pub fn acquire(&self, resource: &str, owner: &str, expire_secs: Option<u64>) -> Result<(), LockError> {
        let mut locks = self.locks.lock().map_err(|e| LockError::Internal(e.to_string()))?;
        if locks.contains_key(resource) {
            return Err(LockError::AlreadyLocked);
        }
        let expire_at = expire_secs.map(|secs| {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            now + secs
        });
        locks.insert(resource.to_string(), LockInfo { owner: owner.to_string(), expire_at });
        drop(locks);
        if let Some(expire_at) = expire_at {
            let mut slots = self.timeslots.lock().unwrap();
            slots.entry(expire_at).or_default().insert(resource.to_string());
        }
        Ok(())
    }

    /// Release a lock for a resource and owner.
    pub fn release(&self, resource: &str, owner: &str) -> Result<(), LockError> {
        let mut locks = self.locks.lock().map_err(|e| LockError::Internal(e.to_string()))?;
        match locks.get(resource) {
            Some(info) if info.owner == owner => {
                // Remove from timeslot if present
                if let Some(expire_at) = info.expire_at {
                    let mut slots = self.timeslots.lock().unwrap();
                    if let Some(set) = slots.get_mut(&expire_at) {
                        set.remove(resource);
                        if set.is_empty() {
                            slots.remove(&expire_at);
                        }
                    }
                }
                locks.remove(resource);
                Ok(())
            }
            Some(_) => Err(LockError::AlreadyLocked),
            None => Err(LockError::NotFound),
        }
    }

    /// Check if a resource is currently locked.
    pub fn is_locked(&self, resource: &str) -> bool {
        let locks = self.locks.lock().unwrap();
        locks.contains_key(resource)
    }

    /// Internal: spawn a background thread to check and release expired locks every second.
    fn spawn_expiry_worker(&self) {
        let locks = self.locks.clone();
        let timeslots = self.timeslots.clone();
        thread::spawn(move || loop {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let expired: Vec<u64> = {
                let slots = timeslots.lock().unwrap();
                slots.keys().filter(|&&ts| ts <= now).cloned().collect()
            };
            for ts in expired {
                let resources = {
                    let mut slots = timeslots.lock().unwrap();
                    slots.remove(&ts).unwrap_or_default()
                };
                let mut l = locks.lock().unwrap();
                for resource in resources {
                    l.remove(&resource);
                }
            }
            thread::sleep(Duration::from_secs(1));
        });
    }
}
