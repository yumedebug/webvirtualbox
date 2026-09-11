export interface VMConfig {
  id: string;
  name: string;
  osType: string;
  cpuCores: number;
  memoryMB: number;
  diskSizeGB: number;
  bootOrder: number; // 0 = CD first, 1 = HDD first
  displayWidth: number;
  displayHeight: number;
  networkEnabled: boolean;
  createdAt: number;
}

export interface VMState {
  running: boolean;
  cpuState: string;
  halted: boolean;
}

export interface WorkerMessage {
  type: string;
  payload?: any;
}
