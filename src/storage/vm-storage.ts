import type { VMConfig } from '../types';

const DB_NAME = 'WebVirtualBox';
const DB_VERSION = 1;
const VM_STORE = 'vms';
const DISK_STORE = 'disks';

export class VMStorage {
  private db: IDBDatabase | null = null;

  async init(): Promise<IDBDatabase> {
    if (this.db) return this.db;
    
    return new Promise((resolve, reject) => {
      const request = indexedDB.open(DB_NAME, DB_VERSION);
      
      request.onupgradeneeded = (event) => {
        const db = (event.target as IDBOpenDBRequest).result;
        if (!db.objectStoreNames.contains(VM_STORE)) {
          db.createObjectStore(VM_STORE, { keyPath: 'id' });
        }
        if (!db.objectStoreNames.contains(DISK_STORE)) {
          db.createObjectStore(DISK_STORE, { keyPath: 'id' });
        }
      };
      
      request.onsuccess = (event) => {
        this.db = (event.target as IDBOpenDBRequest).result;
        resolve(this.db);
      };
      
      request.onerror = () => reject(request.error);
    });
  }

  async saveVM(config: VMConfig): Promise<void> {
    const db = await this.init();
    return new Promise((resolve, reject) => {
      const tx = db.transaction(VM_STORE, 'readwrite');
      const store = tx.objectStore(VM_STORE);
      const request = store.put(config);
      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  }

  async getVM(id: string): Promise<VMConfig | undefined> {
    const db = await this.init();
    return new Promise((resolve, reject) => {
      const tx = db.transaction(VM_STORE, 'readonly');
      const store = tx.objectStore(VM_STORE);
      const request = store.get(id);
      request.onsuccess = () => resolve(request.result);
      request.onerror = () => reject(request.error);
    });
  }

  async getAllVMs(): Promise<VMConfig[]> {
    const db = await this.init();
    return new Promise((resolve, reject) => {
      const tx = db.transaction(VM_STORE, 'readonly');
      const store = tx.objectStore(VM_STORE);
      const request = store.getAll();
      request.onsuccess = () => resolve(request.result || []);
      request.onerror = () => reject(request.error);
    });
  }

  async deleteVM(id: string): Promise<void> {
    const db = await this.init();
    return new Promise((resolve, reject) => {
      const tx = db.transaction(VM_STORE, 'readwrite');
      const store = tx.objectStore(VM_STORE);
      const request = store.delete(id);
      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  }

  async saveDisk(id: string, data: ArrayBuffer): Promise<void> {
    const db = await this.init();
    return new Promise((resolve, reject) => {
      const tx = db.transaction(DISK_STORE, 'readwrite');
      const store = tx.objectStore(DISK_STORE);
      const request = store.put({ id, data });
      request.onsuccess = () => resolve();
      request.onerror = () => reject(request.error);
    });
  }

  async getDisk(id: string): Promise<ArrayBuffer | undefined> {
    const db = await this.init();
    return new Promise((resolve, reject) => {
      const tx = db.transaction(DISK_STORE, 'readonly');
      const store = tx.objectStore(DISK_STORE);
      const request = store.get(id);
      request.onsuccess = () => resolve(request.result?.data);
      request.onerror = () => reject(request.error);
    });
  }
}
