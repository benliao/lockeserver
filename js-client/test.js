const { LockserverClient } = require('./index');

async function main() {
  const client = new LockserverClient({
    addr: '127.0.0.1:8080',
    owner: 'test-worker',
    secret: process.env.LOCKSERVER_SECRET || 'changeme',
  });

  console.log('Acquiring lock...');
  const gotLock = await client.acquire('test-resource', false);
  if (gotLock) {
    console.log('Lock acquired! Doing work...');
    // Simulate work
    await new Promise(r => setTimeout(r, 1000));
    await client.release('test-resource');
    console.log('Lock released.');
  } else {
    console.log('Could not acquire lock (non-blocking).');
  }
}

main().catch(err => {
  console.error('Error:', err);
  process.exit(1);
});
