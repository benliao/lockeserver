from lockserver_client import LockserverClient
import os
import time

if __name__ == "__main__":
    client = LockserverClient(
        addr='127.0.0.1:8080',
        owner='test-worker',
        secret=os.environ.get('LOCKSERVER_SECRET', 'changeme'),
    )
    print('Acquiring lock...')
    got_lock = client.acquire('test-resource', blocking=False)
    if got_lock:
        print('Lock acquired! Doing work...')
        time.sleep(1)
        client.release('test-resource')
        print('Lock released.')
    else:
        print('Could not acquire lock (non-blocking).')
