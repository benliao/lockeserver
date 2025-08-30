//! Lock management for the distributed lock server.
//!
//! This module provides the in-memory lock manager used by the server.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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
#[derive(Debug, Default)]
pub struct LockManager {
    locks: Arc<Mutex<HashMap<String, String>>>, // resource -> owner
}

impl LockManager {
    /// Create a new lock manager.
    pub fn new() -> Self {
        Self { locks: Arc::new(Mutex::new(HashMap::new())) }
    }

    /// Try to acquire a lock for a resource and owner.
    pub fn acquire(&self, resource: &str, owner: &str) -> Result<(), LockError> {
        let mut locks = self.locks.lock().map_err(|e| LockError::Internal(e.to_string()))?;
        if locks.contains_key(resource) {
            Err(LockError::AlreadyLocked)
        } else {
            locks.insert(resource.to_string(), owner.to_string());
            Ok(())
        }
    }

    /// Release a lock for a resource and owner.
    pub fn release(&self, resource: &str, owner: &str) -> Result<(), LockError> {
        let mut locks = self.locks.lock().map_err(|e| LockError::Internal(e.to_string()))?;
        match locks.get(resource) {
            Some(current_owner) if current_owner == owner => {
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
}
