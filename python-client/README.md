## Publishing to PyPI

This package is ready for PyPI. The `setup.py` uses your `README.md` as the long description, so your PyPI page will show this documentation.

To publish:

1. Make sure you have `build` and `twine` installed:
    ```sh
    pip install --upgrade build twine
    ```
2. Build the package:
    ```sh
    python3 -m build
    ```
3. Upload to PyPI:
    ```sh
    python3 -m twine upload dist/*
    ```
4. Check your package at https://pypi.org/project/lockserver-client/

If you update the README, rebuild and re-upload to update the PyPI description.
# lockserver-client (Python)

A Python client SDK for [lockserver](https://github.com/benliao/lockserver), a distributed lock server for coordinating access to shared resources.

## Install

```
pip install requests
```

## Usage

```python
from lockserver_client import LockserverClient

client = LockserverClient(
    addr='127.0.0.1:8080',  # or from env LOCKSERVER_ADDR
    owner='worker1',        # or from env LOCKSERVER_OWNER
    secret='your-strong-secret' # or from env LOCKSERVER_SECRET
)

# Blocking acquire
client.acquire('my-resource')
# ... critical section ...
client.release('my-resource')

# Non-blocking acquire
if client.acquire('my-resource', blocking=False):
    # ... critical section ...
    client.release('my-resource')
```

## Example: Multiple workers writing to S3

```python
from lockserver_client import LockserverClient

def upload_to_s3(bucket, file):
    print(f"Uploading {file} to {bucket}...")

client = LockserverClient(owner='worker-123', secret='your-strong-secret')

if client.acquire('s3-upload-lock'):
    try:
        upload_to_s3('my-bucket', 'file.txt')
    finally:
        client.release('s3-upload-lock')
```

## API

### `LockserverClient(addr=None, owner=None, secret=None)`
- `addr`: lockserver address (default: `127.0.0.1:8080`)
- `owner`: unique owner id (default: `default_owner`)
- `secret`: shared secret (default: `changeme`)

### `acquire(resource, blocking=True)`
- Acquires a lock on `resource`. If `blocking` is False, returns immediately if lock is held.
- Returns `True` (lock acquired) or `False` (non-blocking, lock not acquired).

### `release(resource)`
- Releases the lock on `resource`.

## License

MIT
