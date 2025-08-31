import os
import time
import requests

class LockserverClient:
    def __init__(self, addr=None, owner=None, secret=None):
        self.addr = addr or os.getenv('LOCKSERVER_ADDR', '127.0.0.1:8080')
        self.owner = owner or os.getenv('LOCKSERVER_OWNER', 'default_owner')
        self.secret = secret or os.getenv('LOCKSERVER_SECRET', 'changeme')
        self.base_url = f'http://{self.addr}'

    def acquire(self, resource, blocking=True, expire=None):
        url = f'{self.base_url}/acquire'
        payload = {'resource': resource, 'owner': self.owner}
        if expire is not None:
            payload['expire'] = expire
        headers = {'X-LOCKSERVER-SECRET': self.secret}
        while True:
            resp = requests.post(url, json=payload, headers=headers)
            if resp.status_code == 200:
                return True
            elif resp.status_code == 409:
                if not blocking:
                    return False
                time.sleep(0.2)
            else:
                resp.raise_for_status()

    def release(self, resource):
        url = f'{self.base_url}/release'
        payload = {'resource': resource, 'owner': self.owner}
        headers = {'X-LOCKSERVER-SECRET': self.secret}
        resp = requests.post(url, json=payload, headers=headers)
        resp.raise_for_status()
