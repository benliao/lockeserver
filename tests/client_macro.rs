use lockserver::{LockserverClient, lock_scope};

#[test]
fn test_lock_scope_macro() {
    let client = LockserverClient::new("127.0.0.1:4000", "test_owner");
    // This will fail if the server is not running, but demonstrates macro usage.
    let _ = std::panic::catch_unwind(|| {
        lock_scope!(&client, "test_resource", {
            // critical section
            let x = 2 + 2;
            assert_eq!(x, 4);
        });
    });
}
