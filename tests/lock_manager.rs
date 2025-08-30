use lockserver::LockManager;

#[test]
fn test_acquire_and_release() {
    let manager = LockManager::new();
    assert!(manager.acquire("res1", "owner1").is_ok());
    assert!(manager.is_locked("res1"));
    assert!(manager.release("res1", "owner1").is_ok());
    assert!(!manager.is_locked("res1"));
}

#[test]
fn test_acquire_twice() {
    let manager = LockManager::new();
    assert!(manager.acquire("res1", "owner1").is_ok());
    assert!(manager.acquire("res1", "owner2").is_err());
}

#[test]
fn test_release_wrong_owner() {
    let manager = LockManager::new();
    assert!(manager.acquire("res1", "owner1").is_ok());
    assert!(manager.release("res1", "owner2").is_err());
    assert!(manager.is_locked("res1"));
}
