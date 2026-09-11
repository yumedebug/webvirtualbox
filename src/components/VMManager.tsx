import type { VMConfig } from '../types';

interface Props {
  vms: VMConfig[];
  selectedVM: string | null;
  runningVM: string | null;
  onSelect: (id: string) => void;
  onDelete: (id: string) => void;
  onStart: (id: string) => void;
}

export function VMManager({ vms, selectedVM, runningVM, onSelect, onDelete, onStart }: Props) {
  if (vms.length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center p-4">
        <p className="text-gray-500 text-sm text-center">
          No virtual machines yet.<br />Click "+ New" to create one.
        </p>
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-auto">
      {vms.map(vm => (
        <div
          key={vm.id}
          onClick={() => onSelect(vm.id)}
          className={`p-3 border-b border-gray-700 cursor-pointer transition-colors ${
            selectedVM === vm.id ? 'bg-blue-900/30' : 'hover:bg-gray-800'
          }`}
        >
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <div className={`w-2 h-2 rounded-full ${
                runningVM === vm.id ? 'bg-green-400' : 'bg-gray-500'
              }`} />
              <div>
                <p className="font-medium text-sm">{vm.name}</p>
                <p className="text-xs text-gray-400">{vm.osType}</p>
              </div>
            </div>
            <div className="flex gap-1">
              {runningVM !== vm.id && (
                <button
                  onClick={(e) => { e.stopPropagation(); onStart(vm.id); }}
                  className="text-green-400 hover:text-green-300 p-1 text-xs"
                  title="Start"
                >
                  ▶
                </button>
              )}
              <button
                onClick={(e) => { e.stopPropagation(); onDelete(vm.id); }}
                className="text-red-400 hover:text-red-300 p-1 text-xs"
                title="Delete"
              >
                ✕
              </button>
            </div>
          </div>
          <div className="mt-1 text-xs text-gray-500">
            {vm.cpuCores} CPU • {vm.memoryMB}MB RAM • {vm.diskSizeGB}GB
          </div>
          <div className="mt-0.5 text-xs text-gray-600">
            {runningVM === vm.id ? 'Running' : 'Powered Off'}
          </div>
        </div>
      ))}
    </div>
  );
}
