

# Lockserver

Lockserver is a distributed lock server for coordinating access to shared resources across multiple workers or processes. It provides an HTTP API and a Rust client library with ergonomic macros for easy distributed locking.

## Features
- Simple API for acquiring and releasing locks
- HTTP API only (no TCP service)
- Client library with ergonomic macros (`lock_scope!`)
- Blocking and non-blocking lock acquisition
- Ready for publishing to crates.io

## Usage


### Start the server


You can configure the bind IP and HTTP port using CLI arguments, environment variables, or a `.env` file (using dotenvy):

**CLI arguments (override env/.env):**

```sh
cargo run --release -- --bind 127.0.0.1 --port 9000
# or short form:
cargo run --release -- -b 127.0.0.1 -p 9000
```

**Environment variables or .env file:**

```
LOCKSERVER_BIND_IP=127.0.0.1
LOCKSERVER_PORT=9000
```

Then just run:

```sh
cargo run --release
```



### HTTP API (default port 8080)

- Acquire a lock:
  `POST /acquire` with JSON `{ "resource": "myres", "owner": "worker1" }`
- Release a lock:
  `POST /release` with JSON `{ "resource": "myres", "owner": "worker1" }`

Example using `curl`:

```sh
curl -X POST -H "Content-Type: application/json" -d '{"resource":"myres","owner":"worker1"}' http://localhost:8080/acquire
curl -X POST -H "Content-Type: application/json" -d '{"resource":"myres","owner":"worker1"}' http://localhost:8080/release
```


### Rust Client Library & Macro

Add to your `Cargo.toml`:

```toml
[dependencies]
lockserver = "0.1"
```


#### Client configuration

The client can load the server address and owner from environment variables or a `.env` file:

```
LOCKSERVER_ADDR=127.0.0.1:8080
LOCKSERVER_OWNER=worker1
```

Or pass them directly:

```rust
use lockserver::{LockserverClient, lock_scope};

// Loads from env/.env if None
let client = LockserverClient::new_with_env(None::<String>, None::<String>);
lock_scope!(&client, "resource", {
  // critical section
});

// Override address or owner:
let client = LockserverClient::new_with_env(Some("192.168.1.10:9000"), Some("myworker"));

// Non-blocking mode:
if let Ok(()) = client.acquire_with_mode("resource", lockserver::LockMode::NonBlocking) {
  let _guard = lockserver::LockGuard::new(&client, "resource");
  // critical section
}
```

For more advanced usage, see the integration tests in `tests/lock_scope_macro.rs`.

## License

MIT OR Apache-2.0
