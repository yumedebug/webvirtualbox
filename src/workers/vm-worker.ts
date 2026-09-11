// VM Worker - runs in a separate thread
// Communicates with main thread via postMessage

import { VMCore } from './vm-core';

export interface WorkerMessage {
  type: string;
  payload?: any;
}

let vm: VMCore | null = null;
let running = false;
let frameInterval: number | null = null;

function handleMessage(e: MessageEvent<WorkerMessage>) {
  const { type, payload } = e.data;
  
  switch (type) {
    case 'CREATE_VM':
      vm = new VMCore(payload.memorySize || 16 * 1024 * 1024);
      postMessage({ type: 'VM_CREATED' });
      break;
      
    case 'START_VM':
      if (vm) {
        running = true;
        vm.boot();
        startFrameLoop();
        postMessage({ type: 'VM_STARTED' });
      }
      break;
      
    case 'STOP_VM':
      running = false;
      if (frameInterval !== null) {
        clearInterval(frameInterval);
        frameInterval = null;
      }
      postMessage({ type: 'VM_STOPPED' });
      break;
      
    case 'RESET_VM':
      if (vm) {
        running = false;
        if (frameInterval !== null) {
          clearInterval(frameInterval);
          frameInterval = null;
        }
        vm.reset();
        postMessage({ type: 'VM_RESET' });
      }
      break;
      
    case 'LOAD_BIOS':
      if (vm && payload.data) {
        vm.loadBios(new Uint8Array(payload.data));
        postMessage({ type: 'BIOS_LOADED' });
      }
      break;
      
    case 'LOAD_BOOT_SECTOR':
      if (vm && payload.data) {
        vm.loadBootSector(new Uint8Array(payload.data));
        postMessage({ type: 'BOOT_SECTOR_LOADED' });
      }
      break;
      
    case 'LOAD_ISO':
      if (vm && payload.data) {
        vm.loadISO(new Uint8Array(payload.data));
        postMessage({ type: 'ISO_LOADED' });
      }
      break;
      
    case 'LOAD_DISK':
      if (vm && payload.data) {
        vm.loadDisk(new Uint8Array(payload.data));
        postMessage({ type: 'DISK_LOADED' });
      }
      break;
      
    case 'KEY_DOWN':
      if (vm) {
        vm.keyDown(payload.scancode);
      }
      break;
      
    case 'KEY_UP':
      if (vm) {
        vm.keyUp(payload.scancode);
      }
      break;
      
    case 'MOUSE_MOVE':
      if (vm) {
        vm.mouseMove(payload.x, payload.y);
      }
      break;
      
    case 'MOUSE_BUTTON':
      if (vm) {
        vm.mouseButton(payload.button, payload.pressed);
      }
      break;
      
    case 'GET_STATUS':
      if (vm) {
        postMessage({
          type: 'STATUS',
          payload: {
            cpuState: vm.getCPUState(),
            halted: vm.isHalted(),
            running,
          }
        });
      }
      break;
      
    case 'SET_BOOT_ORDER':
      if (vm) {
        vm.setBootOrder(payload.order);
      }
      break;
      
    case 'STEP':
      if (vm) {
        vm.step();
        postMessage({
          type: 'STEP_DONE',
          payload: { cpuState: vm.getCPUState() }
        });
      }
      break;
  }
}

function startFrameLoop() {
  if (frameInterval !== null) {
    clearInterval(frameInterval);
  }
  
  frameInterval = setInterval(() => {
    if (!vm || !running) return;
    
    vm.runFrame();
    
    // Send framebuffer to main thread
    const fb = vm.getFramebuffer();
    const width = vm.getFramebufferWidth();
    const height = vm.getFramebufferHeight();
    
    postMessage({
      type: 'FRAME',
      payload: {
        framebuffer: Array.from(fb),
        width,
        height,
      }
    });
    
  }, 33); // ~30 FPS
}

self.onmessage = handleMessage;
