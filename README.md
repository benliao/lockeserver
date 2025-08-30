
# Lockserver

A distributed lock server for coordinating access to shared resources.

## Features
- Simple API for acquiring and releasing locks
- TCP and HTTP API
- Client library with ergonomic macros
- Ready for publishing to crates.io

## Usage

### Start the server

```sh
cargo run --release
```

### TCP API (default port 4000)

- Acquire a lock:
  `ACQUIRE resource_name owner_id\n`
- Release a lock:
  `RELEASE resource_name owner_id\n`

Example using `nc`:

```sh
echo "ACQUIRE myres worker1" | nc localhost 4000
echo "RELEASE myres worker1" | nc localhost 4000
```

### HTTP API (port 8080)

- Acquire a lock:
  `POST /acquire` with JSON `{ "resource": "myres", "owner": "worker1" }`
- Release a lock:
  `POST /release` with JSON `{ "resource": "myres", "owner": "worker1" }`

Example using `curl`:

```sh
curl -X POST -H "Content-Type: application/json" -d '{"resource":"myres","owner":"worker1"}' http://localhost:8080/acquire
curl -X POST -H "Content-Type: application/json" -d '{"resource":"myres","owner":"worker1"}' http://localhost:8080/release
```

### Client Library & Macro

Add to your `Cargo.toml`:

```toml
[dependencies]
lockserver = "0.1"
```

Example usage:

```rust
use lockserver::{LockserverClient, lock_scope};

let client = LockserverClient::new("127.0.0.1:4000", "worker1");
lock_scope!(&client, "resource", {
    // critical section
});

// Non-blocking mode:
if let Ok(()) = client.acquire_with_mode("resource", lockserver::LockMode::NonBlocking) {
    let _guard = lockserver::LockGuard::new(&client, "resource");
    // critical section
}
```

## License
MIT OR Apache-2.0
