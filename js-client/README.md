# lockserver-client (Node.js)

A Node.js client SDK for [lockserver](https://github.com/benliao/lockserver), a distributed lock server for coordinating access to shared resources.

## Install

```
npm install lockserver-client
```


## Usage

### Example: Multiple workers writing to S3

```js
const { LockserverClient } = require('lockserver-client');

async function uploadToS3(bucket, file) {
  // Your S3 upload logic here
  console.log(`Uploading ${file} to ${bucket}...`);
}

async function main() {
  const client = new LockserverClient({
    addr: '127.0.0.1:8080', // or from env LOCKSERVER_ADDR
    owner: 'worker1',       // or from env LOCKSERVER_OWNER
    secret: 'your-strong-secret' // or from env LOCKSERVER_SECRET
  });

  // Acquire lock before writing to S3
  await client.acquire('s3-upload-lock');
  try {
    await uploadToS3('my-bucket', 'file.txt');
  } finally {
    await client.release('s3-upload-lock');
  }
}

main();
```

### Basic usage

```js
const { LockserverClient } = require('lockserver-client');
const client = new LockserverClient({ addr: '127.0.0.1:8080', owner: 'worker1', secret: 'your-strong-secret' });

// Blocking acquire
await client.acquire('my-resource');
// ... critical section ...
await client.release('my-resource');

// Non-blocking acquire
const gotLock = await client.acquire('my-resource', false);
if (gotLock) {
  // ... critical section ...
  await client.release('my-resource');
}
```
## Publishing

To publish this SDK to npm:

1. Update the version in `package.json` as needed.
2. Run `npm login` if you haven't already.
3. Run `npm publish` from the `js-client` directory.

See [npm docs](https://docs.npmjs.com/cli/v10/commands/npm-publish) for more details.

## API

### `new LockserverClient(options)`
- `options.addr` - lockserver address (default: `127.0.0.1:8080`)
- `options.owner` - unique owner id (default: `default_owner`)
- `options.secret` - shared secret (default: `changeme`)

### `acquire(resource, blocking = true, expire)`
- Acquires a lock on `resource`. If `blocking` is false, returns immediately if lock is held.
- `expire` (optional): lock expiration in seconds. After this time, the lock is auto-released by the server.
- Returns a Promise resolving to `true` (lock acquired) or `false` (non-blocking, lock not acquired).

### `release(resource)`
- Releases the lock on `resource`.

## License

MIT
