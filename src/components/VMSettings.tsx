import { useState } from 'react';
import type { VMConfig } from '../types';

interface Props {
  vm: VMConfig;
  onClose: () => void;
  onSave: (config: VMConfig) => void;
}

export function VMSettings({ vm, onClose, onSave }: Props) {
  const [name, setName] = useState(vm.name);
  const [cpuCores, setCpuCores] = useState(vm.cpuCores);
  const [memoryMB, setMemoryMB] = useState(vm.memoryMB);
  const [diskSizeGB, setDiskSizeGB] = useState(vm.diskSizeGB);
  const [bootOrder, setBootOrder] = useState(vm.bootOrder);

  const handleSave = () => {
    onSave({
      ...vm,
      name: name.trim() || vm.name,
      cpuCores,
      memoryMB,
      diskSizeGB,
      bootOrder,
    });
  };

  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg w-full max-w-lg mx-4">
        <div className="p-6">
          <h2 className="text-xl font-bold mb-6">VM Settings - {vm.name}</h2>
          
          <div className="space-y-4">
            <div>
              <label className="block text-sm text-gray-300 mb-1">Name</label>
              <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white focus:outline-none focus:border-blue-500"
              />
            </div>

            <div>
              <label className="block text-sm text-gray-300 mb-1">CPU Cores: {cpuCores}</label>
              <input
                type="range"
                min={1}
                max={4}
                value={cpuCores}
                onChange={(e) => setCpuCores(Number(e.target.value))}
                className="w-full"
              />
            </div>

            <div>
              <label className="block text-sm text-gray-300 mb-1">Memory: {memoryMB} MB</label>
              <input
                type="range"
                min={128}
                max={2048}
                step={128}
                value={memoryMB}
                onChange={(e) => setMemoryMB(Number(e.target.value))}
                className="w-full"
              />
            </div>

            <div>
              <label className="block text-sm text-gray-300 mb-1">Disk Size: {diskSizeGB} GB</label>
              <input
                type="range"
                min={1}
                max={100}
                value={diskSizeGB}
                onChange={(e) => setDiskSizeGB(Number(e.target.value))}
                className="w-full"
              />
            </div>

            <div>
              <label className="block text-sm text-gray-300 mb-1">Boot Order</label>
              <select
                value={bootOrder}
                onChange={(e) => setBootOrder(Number(e.target.value))}
                className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white focus:outline-none focus:border-blue-500"
              >
                <option value={0}>CD/DVD → Hard Disk</option>
                <option value={1}>Hard Disk → CD/DVD</option>
              </select>
            </div>
          </div>

          <div className="flex justify-end gap-3 mt-6 pt-4 border-t border-gray-700">
            <button
              onClick={onClose}
              className="px-4 py-2 text-gray-300 hover:text-white transition-colors"
            >
              Cancel
            </button>
            <button
              onClick={handleSave}
              className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded transition-colors"
            >
              Save
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
