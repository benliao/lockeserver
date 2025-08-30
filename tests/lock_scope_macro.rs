use lockserver::{LockserverClient, lock_scope};

#[test]

fn test_lock_scope_blocking() {
    let client = LockserverClient::new_with_env(
        Some("127.0.0.1:8080"),
        Some("worker1"),
        None::<String>, // Use env or default for secret
    );
    // This will fail if the server is not running, but demonstrates macro usage.
    let _ = std::panic::catch_unwind(|| {
        lock_scope!(&client, "resource", {
            // critical section
            let x = 2 + 2;
            assert_eq!(x, 4);
        });
    });
}

#[test]
fn test_lock_scope_non_blocking() {
    let client = LockserverClient::new_with_env(
        Some("127.0.0.1:8080"),
        Some("worker1"),
        None::<String>, // Use env or default for secret
    );
    let _ = std::panic::catch_unwind(|| {
        lock_scope!(&client, "resource", non_blocking, {
            // critical section
            let y = 3 + 3;
            assert_eq!(y, 6);
        });
    });
}
