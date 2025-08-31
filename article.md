## Five Suggested Topics for Your Medium Article

1. **Why Distributed Locking Matters: Real-World Use Cases and Challenges**
2. **Building a Secure, Minimal Distributed Lock Server in Rust**
3. **HTTP-Only API and Multi-Language SDKs: Making Distributed Locks Accessible**
4. **Securing Distributed Coordination with Shared Secrets**
5. **From Rust to Node.js and Python: Lessons in Multi-Language Open Source**

---

How I Solved S3 Write Contention with a Distributed Lock Server
By Ben Liao
Introduction

When building distributed systems, it's common to have multiple workers or services that need to write to a shared resource - like an Amazon S3 bucket. However, S3 does not provide native locking, and concurrent writes can lead to data corruption, race conditions, or unexpected results. In this article, I'll walk you through how I solved this problem by building and deploying a distributed lock server, and how you can do the same.
The Problem: Concurrent Writes to S3

Imagine you have several distributed workers, each responsible for uploading files to the same S3 bucket. If two workers try to write the same file at the same time, you risk:
Data corruption or partial writes
Inconsistent state
Difficult-to-debug race conditions

A robust solution requires a distributed lock - something that all workers can check before writing.
Step 1: Evaluating Locking Strategies

I considered several approaches:
S3 Object Locking: Not suitable for all use cases and can be complex to manage.
DynamoDB-based locks: Reliable, but adds cost and complexity.
Redis or Zookeeper: Powerful, but overkill for simple lock coordination.

I wanted something lightweight, language-agnostic, and easy to deploy.
Step 2: Building a Distributed Lock Server

I decided to build a dedicated lock server, inspired by the simplicity of Redis' SETNX but with a RESTful HTTP API and client libraries for multiple languages.
Key features:
HTTP API for acquiring and releasing locks
Shared secret for secure access
Client SDKs for Rust, Python, and Node.js
Blocking and non-blocking lock acquisition

Step 3: Implementing the Lock Server

I implemented the server in Rust for performance and reliability. The server exposes two endpoints:
POST /acquire - Try to acquire a lock for a resource/owner
POST /release - Release the lock

Each request must include a shared secret for authorization.
Step 4: Integrating the Client in My Workers

I created lightweight client SDKs for Python and Node.js, making it easy to add distributed locking to any worker.
Python Example:
from lockserver_client import Lockserver
Clientclient = LockserverClient(owner='worker-123', secret='your-strong-secret')
if client.acquire('s3-upload-lock'):
  try:
    upload_to_s3('my-bucket', 'file.txt') 
  finally:
    client.release('s3-upload-lock')
Node.js Example:
const { LockserverClient } = require('lockserver-client');
const client = new LockserverClient({ owner: 'worker-123', secret: 'your-strong-secret' });
await client.acquire('s3-upload-lock');
// … upload to S3 …
await client.release('s3-upload-lock');

Step 5: Deploying and Testing

* Deployed the lockserver as a standalone service (Docker, bare metal, or cloud VM)
* Configured all workers to use the same LOCKSERVER_SECRET
* Verified that only one worker could write to S3 at a time for the same resource
Results

* No more race conditions: Only one worker writes at a time.
* Language-agnostic: Any worker (Rust, Python, Node.js, etc.) can participate.
* Simple integration: Just a few lines of code to add distributed locking.

Conclusion

By introducing a distributed lock server, I was able to solve S3 write contention in a scalable, secure, and language-agnostic way. If you're facing similar challenges, consider using or contributing to lockserver.
Links:
GitHub: https://github.com/benliao/lockserver
Crate: https://crates.io/crates/lockserver
npm: https://www.npmjs.com/package/lockserver-client

Thanks for reading! If you found this helpful, feel free to star the repo or leave a comment below.
