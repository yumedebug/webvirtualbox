import { useEffect, useRef, useState, useCallback } from 'react';
import type { VMConfig } from '../types';
import { VMCore } from '../workers/vm-core';
import { VMStorage } from '../storage/vm-storage';

interface Props {
  vm: VMConfig;
  onStop: () => void;
  onReset: () => void;
}

export function VMConsole({ vm, onStop, onReset }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const vmRef = useRef<VMCore | null>(null);
  const runningRef = useRef(false);
  const animFrameRef = useRef<number>(0);
  const [cpuState, setCpuState] = useState('');
  const [isHalted, setIsHalted] = useState(false);
  const frameCountRef = useRef(0);

  useEffect(() => {
    let disposed = false;
    // Initialize VM
    const memorySize = vm.memoryMB * 1024 * 1024;
    const vmCore = new VMCore(Math.min(memorySize, 16 * 1024 * 1024)); // Cap at 16MB for browser
    vmCore.setBootOrder(vm.bootOrder);

    const start = async () => {
      const storage = new VMStorage();
      const iso = await storage.getDisk(`${vm.id}:iso`);
      if (disposed) return;
      if (iso) vmCore.loadISO(new Uint8Array(iso));
      vmCore.boot();
      vmCore.bootFromStorage();
      vmRef.current = vmCore;
      runningRef.current = true;
      animFrameRef.current = requestAnimationFrame(runLoop);
    };
    void start();

    const runLoop = () => {
      if (!runningRef.current || !vmRef.current) return;
      
      vmRef.current.runFrame();
      frameCountRef.current++;
      
      // Update display
      const canvas = canvasRef.current;
      if (canvas) {
        const ctx = canvas.getContext('2d');
        if (ctx) {
          const fb = vmRef.current.getFramebuffer();
          const width = vmRef.current.getFramebufferWidth();
          const height = vmRef.current.getFramebufferHeight();
          
          const imageData = ctx.createImageData(width, height);
          for (let i = 0; i < fb.length && i < imageData.data.length; i++) {
            imageData.data[i] = fb[i];
          }
          ctx.putImageData(imageData, 0, 0);
        }
      }
      
      // Update CPU state periodically
      if (frameCountRef.current % 10 === 0) {
        setCpuState(vmRef.current.getCPUState());
        setIsHalted(vmRef.current.isHalted());
      }
      
      animFrameRef.current = requestAnimationFrame(runLoop);
    };

    return () => {
      disposed = true;
      runningRef.current = false;
      if (animFrameRef.current) {
        cancelAnimationFrame(animFrameRef.current);
      }
    };
  }, [vm.id]);

  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    if (!vmRef.current) return;
    e.preventDefault();
    const scancode = keyToScancode(e.code);
    if (scancode !== null) {
      vmRef.current.keyDown(scancode);
    }
  }, []);

  const handleKeyUp = useCallback((e: React.KeyboardEvent) => {
    if (!vmRef.current) return;
    e.preventDefault();
    const scancode = keyToScancode(e.code);
    if (scancode !== null) {
      vmRef.current.keyUp(scancode);
    }
  }, []);

  const handleMouseMove = useCallback((e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!vmRef.current || !canvasRef.current) return;
    const rect = canvasRef.current.getBoundingClientRect();
    const x = Math.floor((e.clientX - rect.left) / rect.width * 640);
    const y = Math.floor((e.clientY - rect.top) / rect.height * 480);
    vmRef.current.mouseMove(x, y);
  }, []);

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    if (!vmRef.current) return;
    vmRef.current.mouseButton(e.button, true);
  }, []);

  const handleMouseUp = useCallback((e: React.MouseEvent) => {
    if (!vmRef.current) return;
    vmRef.current.mouseButton(e.button, false);
  }, []);

  return (
    <div className="flex-1 flex flex-col overflow-hidden">
      {/* Toolbar */}
      <div className="bg-gray-800 border-b border-gray-700 px-4 py-2 flex items-center justify-between shrink-0">
        <div className="flex items-center gap-3">
          <h2 className="font-medium">{vm.name}</h2>
          <span className={`text-xs px-2 py-0.5 rounded ${
            isHalted ? 'bg-red-900 text-red-300' : 'bg-green-900 text-green-300'
          }`}>
            {isHalted ? 'Halted' : 'Running'}
          </span>
        </div>
        <div className="flex gap-2">
          <button
            onClick={onReset}
            className="bg-yellow-600 hover:bg-yellow-700 text-white text-sm px-3 py-1 rounded transition-colors"
          >
            ↻ Reset
          </button>
          <button
            onClick={onStop}
            className="bg-red-600 hover:bg-red-700 text-white text-sm px-3 py-1 rounded transition-colors"
          >
            ■ Stop
          </button>
        </div>
      </div>

      {/* VM Display */}
      <div className="flex-1 flex items-center justify-center bg-black p-4 overflow-auto">
        <canvas
          ref={canvasRef}
          width={640}
          height={480}
          tabIndex={0}
          onKeyDown={handleKeyDown}
          onKeyUp={handleKeyUp}
          onMouseMove={handleMouseMove}
          onMouseDown={handleMouseDown}
          onMouseUp={handleMouseUp}
          className="border border-gray-700 max-w-full max-h-full"
          style={{ imageRendering: 'pixelated' }}
        />
      </div>

      {/* Status bar */}
      <div className="bg-gray-800 border-t border-gray-700 px-4 py-2 text-xs text-gray-400 flex items-center justify-between shrink-0">
        <div className="flex gap-4">
          <span>CPU: {vm.cpuCores} cores</span>
          <span>RAM: {vm.memoryMB} MB</span>
          <span>Display: {vm.displayWidth}×{vm.displayHeight}</span>
        </div>
        <div className="font-mono text-xs truncate max-w-md">
          {cpuState.split('\n')[0] || 'Initializing...'}
        </div>
      </div>
    </div>
  );
}

function keyToScancode(code: string): number | null {
  const keyMap: Record<string, number> = {
    'Escape': 0x01,
    'Digit1': 0x02, 'Digit2': 0x03, 'Digit3': 0x04, 'Digit4': 0x05,
    'Digit5': 0x06, 'Digit6': 0x07, 'Digit7': 0x08, 'Digit8': 0x09,
    'Digit9': 0x0A, 'Digit0': 0x0B,
    'Minus': 0x0C, 'Equal': 0x0D, 'Backspace': 0x0E,
    'Tab': 0x0F,
    'KeyQ': 0x10, 'KeyW': 0x11, 'KeyE': 0x12, 'KeyR': 0x13,
    'KeyT': 0x14, 'KeyY': 0x15, 'KeyU': 0x16, 'KeyI': 0x17,
    'KeyO': 0x18, 'KeyP': 0x19,
    'BracketLeft': 0x1A, 'BracketRight': 0x1B,
    'Enter': 0x1C,
    'ControlLeft': 0x1D,
    'KeyA': 0x1E, 'KeyS': 0x1F, 'KeyD': 0x20, 'KeyF': 0x21,
    'KeyG': 0x22, 'KeyH': 0x23, 'KeyJ': 0x24, 'KeyK': 0x25,
    'KeyL': 0x26, 'Semicolon': 0x27, 'Quote': 0x28,
    'Backquote': 0x29,
    'ShiftLeft': 0x2A,
    'Backslash': 0x2B,
    'KeyZ': 0x2C, 'KeyX': 0x2D, 'KeyC': 0x2E, 'KeyV': 0x2F,
    'KeyB': 0x30, 'KeyN': 0x31, 'KeyM': 0x32,
    'Comma': 0x33, 'Period': 0x34, 'Slash': 0x35,
    'ShiftRight': 0x36,
    'NumpadMultiply': 0x37,
    'AltLeft': 0x38,
    'Space': 0x39,
    'CapsLock': 0x3A,
    'F1': 0x3B, 'F2': 0x3C, 'F3': 0x3D, 'F4': 0x3E,
    'F5': 0x3F, 'F6': 0x40, 'F7': 0x41, 'F8': 0x42,
    'F9': 0x43, 'F10': 0x44,
    'Numpad7': 0x47, 'Numpad8': 0x48, 'Numpad9': 0x49,
    'NumpadSubtract': 0x4A,
    'Numpad4': 0x4B, 'Numpad5': 0x4C, 'Numpad6': 0x4D,
    'NumpadAdd': 0x4E,
    'Numpad1': 0x4F, 'Numpad2': 0x50, 'Numpad3': 0x51,
    'Numpad0': 0x52, 'NumpadDecimal': 0x53,
    'F11': 0x57, 'F12': 0x58,
    'ArrowUp': 0x48,
    'ArrowDown': 0x50,
    'ArrowLeft': 0x4B,
    'ArrowRight': 0x4D,
  };
  
  return keyMap[code] ?? null;
}
