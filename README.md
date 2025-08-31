# Lockserver

![Crates.io](https://img.shields.io/crates/v/lockserver)
![Docs.rs](https://img.shields.io/docsrs/lockserver)
![npm](https://img.shields.io/npm/v/lockserver-client)
![PyPI](https://img.shields.io/pypi/v/lockserver-client)
![CI](https://github.com/benliao/lockserver/actions/workflows/ci.yml/badge.svg)
![License](https://img.shields.io/crates/l/lockserver)

Lockserver is a distributed lock server for coordinating access to shared resources across multiple workers or processes. It provides an HTTP API and official client libraries for Rust (with ergonomic macros), Node.js, and Python for easy distributed locking in any environment.

## Features
- Simple API for acquiring and releasing locks
- Official client libraries for Rust, Node.js, and Python
- Ergonomic Rust macros (`lock_scope!`)
- Blocking and non-blocking lock acquisition
- **Lock expiration:** Optionally set an expiration (in seconds) when acquiring a lock; expired locks are auto-released
  
## Security: Shared Secret Authorization

All client requests must include a shared secret for authorization. The server and client must agree on the same secret, set via the `LOCKSERVER_SECRET` environment variable or `.env` file. The client sends this secret in the `X-LOCKSERVER-SECRET` HTTP header.

**Example .env:**
```
LOCKSERVER_SECRET=your-strong-secret
```

**Server:**
```
LOCKSERVER_SECRET=your-strong-secret
cargo run --release
```

**Client:**
```
LOCKSERVER_SECRET=your-strong-secret
```

Or pass the secret directly to the client constructor.

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
  `POST /acquire` with JSON `{ "resource": "myres", "owner": "worker1" [, "expire": 10] }`
  - Optional `expire` (seconds): lock will be auto-released after this many seconds
- Release a lock:
  `POST /release` with JSON `{ "resource": "myres", "owner": "worker1" }`

Example using `curl` (with secret and expiration):

```sh
curl -X POST -H "Content-Type: application/json" -H "X-LOCKSERVER-SECRET: your-strong-secret" \
  -d '{"resource":"myres","owner":"worker1","expire":10}' http://localhost:8080/acquire
curl -X POST -H "Content-Type: application/json" -H "X-LOCKSERVER-SECRET: your-strong-secret" \
  -d '{"resource":"myres","owner":"worker1"}' http://localhost:8080/release
```



## Client SDKs

- **Rust**: See below and the integration tests in `tests/lock_scope_macro.rs`.
  - `acquire_with_mode_and_expire(resource, mode, expire)` allows setting expiration in seconds
- **Node.js**: [js-client/](js-client/) ([npm](https://www.npmjs.com/package/lockserver-client))
  - `acquire(resource, blocking = true, expire)` supports expiration (in seconds)
- **Python**: [python-client/](python-client/) ([PyPI](https://pypi.org/project/lockserver-client/))
  - `acquire(resource, blocking=True, expire=None)` supports expiration (in seconds)

### Rust Example

Add to your `Cargo.toml`:

```toml
[dependencies]
lockserver = "0.1"
```

```rust
use lockserver::{LockserverClient, lock_scope};

// Loads from env/.env if None (including secret)
let client = LockserverClient::new_with_env(None::<String>, None::<String>, None::<String>);
lock_scope!(&client, "resource", {
  // critical section
});

// Override address, owner, or secret:
let client = LockserverClient::new_with_env(
    Some("192.168.1.10:9000"),
    Some("myworker"),
    Some("your-strong-secret")
);

// Non-blocking mode:
if let Ok(()) = client.acquire_with_mode("resource", lockserver::LockMode::NonBlocking) {
  let _guard = lockserver::LockGuard::new(&client, "resource");
  // critical section
}

// With expiration:
if let Ok(()) = client.acquire_with_mode_and_expire("resource", lockserver::LockMode::NonBlocking, 10) {
  let _guard = lockserver::LockGuard::new(&client, "resource");
  // critical section
}
```

See the respective `README.md` in each client directory for Node.js and Python usage and installation instructions.

## License

Licensed under the [MIT License](LICENSE).
