import { useState, useRef } from 'react';
import type { VMConfig } from '../types';

interface Props {
  onClose: () => void;
  onCreate: (config: VMConfig, isoData?: ArrayBuffer) => void;
}

export function VMCreateDialog({ onClose, onCreate }: Props) {
  const [name, setName] = useState('');
  const [osType, setOsType] = useState('Linux');
  const [cpuCores, setCpuCores] = useState(1);
  const [memoryMB, setMemoryMB] = useState(512);
  const [diskSizeGB, setDiskSizeGB] = useState(10);
  const [bootOrder, setBootOrder] = useState(0);
  const [isoFile, setIsoFile] = useState<File | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleCreate = () => {
    if (!name.trim()) return;
    
    const config: VMConfig = {
      id: crypto.randomUUID(),
      name: name.trim(),
      osType,
      cpuCores,
      memoryMB,
      diskSizeGB,
      bootOrder,
      displayWidth: 640,
      displayHeight: 480,
      networkEnabled: false,
      createdAt: Date.now(),
      isoName: isoFile?.name,
    };

    if (isoFile) {
      isoFile.arrayBuffer().then(data => onCreate(config, data));
    } else {
      onCreate(config);
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      setIsoFile(e.target.files[0]);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg w-full max-w-lg mx-4 max-h-[90vh] overflow-auto">
        <div className="p-6">
          <h2 className="text-xl font-bold mb-6">Create Virtual Machine</h2>
          
          <div className="space-y-4">
            {/* Name */}
            <div>
              <label className="block text-sm text-gray-300 mb-1">Name</label>
              <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="My Virtual Machine"
                className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white placeholder-gray-500 focus:outline-none focus:border-blue-500"
              />
            </div>

            {/* OS Type */}
            <div>
              <label className="block text-sm text-gray-300 mb-1">Guest OS Type</label>
              <select
                value={osType}
                onChange={(e) => setOsType(e.target.value)}
                className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white focus:outline-none focus:border-blue-500"
              >
                <option value="Linux">Linux</option>
                <option value="Windows">Windows</option>
                <option value="Other">Other</option>
              </select>
            </div>

            {/* ISO */}
            <div>
              <label className="block text-sm text-gray-300 mb-1">ISO Image (optional)</label>
              <input
                ref={fileInputRef}
                type="file"
                accept=".iso,.img"
                onChange={handleFileChange}
                className="hidden"
              />
              <button
                onClick={() => fileInputRef.current?.click()}
                className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-left text-gray-300 hover:border-blue-500 transition-colors"
              >
                {isoFile ? isoFile.name : 'Select ISO file...'}
              </button>
            </div>

            {/* CPU */}
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

            {/* Memory */}
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
              <div className="flex justify-between text-xs text-gray-500 mt-1">
                <span>128 MB</span>
                <span>2048 MB</span>
              </div>
            </div>

            {/* Disk */}
            <div>
              <label className="block text-sm text-gray-300 mb-1">Virtual Disk: {diskSizeGB} GB</label>
              <input
                type="range"
                min={1}
                max={100}
                value={diskSizeGB}
                onChange={(e) => setDiskSizeGB(Number(e.target.value))}
                className="w-full"
              />
              <div className="flex justify-between text-xs text-gray-500 mt-1">
                <span>1 GB</span>
                <span>100 GB</span>
              </div>
            </div>

            {/* Boot Order */}
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

            {/* Display */}
            <div>
              <label className="block text-sm text-gray-300 mb-1">Display Resolution</label>
              <select className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white focus:outline-none focus:border-blue-500">
                <option>640 × 480</option>
                <option>800 × 600</option>
                <option>1024 × 768</option>
              </select>
            </div>

            {/* Network */}
            <div>
              <label className="block text-sm text-gray-300 mb-1">Network</label>
              <select className="w-full bg-gray-700 border border-gray-600 rounded px-3 py-2 text-white focus:outline-none focus:border-blue-500">
                <option>Disabled</option>
                <option>NAT (Not available)</option>
              </select>
            </div>
          </div>

          {/* Buttons */}
          <div className="flex justify-end gap-3 mt-6 pt-4 border-t border-gray-700">
            <button
              onClick={onClose}
              className="px-4 py-2 text-gray-300 hover:text-white transition-colors"
            >
              Cancel
            </button>
            <button
              onClick={handleCreate}
              disabled={!name.trim()}
              className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white px-6 py-2 rounded transition-colors"
            >
              Create
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
