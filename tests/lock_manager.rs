use lockserver::LockManager;

#[test]
fn test_acquire_and_release() {
    let manager = LockManager::new();
    assert!(manager.acquire("res1", "owner1", None).is_ok());
    assert!(manager.is_locked("res1"));
    assert!(manager.release("res1", "owner1").is_ok());
    assert!(!manager.is_locked("res1"));
}

#[test]
fn test_expire_lock() {
    let manager = LockManager::new();
    // Acquire with 2 second expiration
    assert!(manager.acquire("res_exp", "owner_exp", Some(2)).is_ok());
    assert!(manager.is_locked("res_exp"));
    // Wait 3 seconds
    std::thread::sleep(std::time::Duration::from_secs(3));
    // Should be auto-released
    assert!(!manager.is_locked("res_exp"));
}

#[test]
fn test_acquire_twice() {
    let manager = LockManager::new();
    assert!(manager.acquire("res1", "owner1", None).is_ok());
    assert!(manager.acquire("res1", "owner2", None).is_err());
}

#[test]
fn test_release_wrong_owner() {
    let manager = LockManager::new();
    assert!(manager.acquire("res1", "owner1", None).is_ok());
    assert!(manager.release("res1", "owner2").is_err());
    assert!(manager.is_locked("res1"));
}
