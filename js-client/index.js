const axios = require('axios');

class LockserverClient {
  /**
   * @param {Object} options
   * @param {string} options.addr - e.g. '127.0.0.1:8080'
   * @param {string} options.owner - unique owner id for this client
   * @param {string} options.secret - shared secret for authorization
   */
  constructor({ addr, owner, secret }) {
    this.addr = addr || process.env.LOCKSERVER_ADDR || '127.0.0.1:8080';
    this.owner = owner || process.env.LOCKSERVER_OWNER || 'default_owner';
    this.secret = secret || process.env.LOCKSERVER_SECRET || 'changeme';
    this.baseUrl = `http://${this.addr}`;
  }

  async acquire(resource, blocking = true) {
    const url = `${this.baseUrl}/acquire`;
    const payload = { resource, owner: this.owner };
    while (true) {
      try {
        await axios.post(url, payload, {
          headers: { 'X-LOCKSERVER-SECRET': this.secret }
        });
        return true;
      } catch (err) {
        if (err.response && err.response.status === 409) {
          if (!blocking) return false;
          await new Promise(r => setTimeout(r, 200));
        } else {
          throw err;
        }
      }
    }
  }

  async release(resource) {
    const url = `${this.baseUrl}/release`;
    const payload = { resource, owner: this.owner };
    await axios.post(url, payload, {
      headers: { 'X-LOCKSERVER-SECRET': this.secret }
    });
  }
}

module.exports = { LockserverClient };
