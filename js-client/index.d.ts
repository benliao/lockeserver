export interface LockserverClientOptions {
  addr?: string;
  owner?: string;
  secret?: string;
}

export class LockserverClient {
  constructor(options: LockserverClientOptions);
  acquire(resource: string, blocking?: boolean, expire?: number): Promise<boolean>;
  release(resource: string): Promise<void>;
}
