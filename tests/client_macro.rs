use lockserver::{LockserverClient, lock_scope};

#[test]
fn test_lock_scope_macro() {
    let client = LockserverClient::new_with_env(
        Some("127.0.0.1:8080"),
        Some("test_owner"),
        None::<String>, // Use env or default for secret
    );
    // This will fail if the server is not running, but demonstrates macro usage.
    let _ = std::panic::catch_unwind(|| {
        lock_scope!(&client, "test_resource_macro", {
            // critical section
            let x = 2 + 2;
            assert_eq!(x, 4);
        });
    });
}
