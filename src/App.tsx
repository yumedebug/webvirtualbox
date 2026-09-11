import { useState, useEffect } from 'react';
import { VMManager } from './components/VMManager';
import { VMConsole } from './components/VMConsole';
import { VMCreateDialog } from './components/VMCreateDialog';
import { VMSettings } from './components/VMSettings';
import { VMStorage } from './storage/vm-storage';
import type { VMConfig } from './types';

export default function App() {
  const [vms, setVMs] = useState<VMConfig[]>([]);
  const [selectedVM, setSelectedVM] = useState<string | null>(null);
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [showSettings, setShowSettings] = useState(false);
  const [runningVM, setRunningVM] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadVMs();
  }, []);

  const loadVMs = async () => {
    const storage = new VMStorage();
    const loaded = await storage.getAllVMs();
    setVMs(loaded);
    setLoading(false);
  };

  const handleCreateVM = async (config: VMConfig) => {
    const storage = new VMStorage();
    await storage.saveVM(config);
    setVMs([...vms, config]);
    setShowCreateDialog(false);
  };

  const handleDeleteVM = async (id: string) => {
    const storage = new VMStorage();
    await storage.deleteVM(id);
    setVMs(vms.filter(vm => vm.id !== id));
    if (selectedVM === id) setSelectedVM(null);
    if (runningVM === id) setRunningVM(null);
  };

  const handleStartVM = (id: string) => {
    setRunningVM(id);
    setSelectedVM(id);
  };

  const handleStopVM = () => {
    setRunningVM(null);
  };

  const handleResetVM = () => {
    if (runningVM) {
      setRunningVM(null);
      setTimeout(() => setRunningVM(runningVM), 100);
    }
  };

  const currentVM = vms.find(vm => vm.id === selectedVM);

  if (loading) {
    return (
      <div className="h-screen bg-gray-900 flex items-center justify-center">
        <div className="text-white text-xl">Loading WebVirtualBox...</div>
      </div>
    );
  }

  return (
    <div className="h-screen bg-gray-900 text-white flex flex-col overflow-hidden">
      {/* Header */}
      <header className="bg-gray-800 border-b border-gray-700 px-4 py-2 flex items-center justify-between shrink-0">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 bg-blue-600 rounded flex items-center justify-center font-bold text-sm">
            VB
          </div>
          <h1 className="text-lg font-semibold">WebVirtualBox</h1>
        </div>
        <div className="text-sm text-gray-400">
          Browser-based Virtual Machine
        </div>
      </header>

      {/* Main content */}
      <div className="flex flex-1 overflow-hidden">
        {/* Left panel - VM List */}
        <aside className="w-72 bg-gray-850 border-r border-gray-700 flex flex-col shrink-0" style={{backgroundColor: '#1a1f2e'}}>
          <div className="p-3 border-b border-gray-700 flex items-center justify-between">
            <h2 className="font-medium text-sm text-gray-300">Virtual Machines</h2>
            <button
              onClick={() => setShowCreateDialog(true)}
              className="bg-blue-600 hover:bg-blue-700 text-white text-xs px-3 py-1.5 rounded transition-colors"
            >
              + New
            </button>
          </div>
          <VMManager
            vms={vms}
            selectedVM={selectedVM}
            runningVM={runningVM}
            onSelect={setSelectedVM}
            onDelete={handleDeleteVM}
            onStart={handleStartVM}
          />
        </aside>

        {/* Right panel - VM Console / Details */}
        <main className="flex-1 flex flex-col overflow-hidden">
          {runningVM && currentVM ? (
            <VMConsole
              vm={currentVM}
              onStop={handleStopVM}
              onReset={handleResetVM}
            />
          ) : currentVM ? (
            <div className="flex-1 p-6 overflow-auto">
              <div className="max-w-2xl mx-auto">
                <h2 className="text-2xl font-bold mb-4">{currentVM.name}</h2>
                <div className="bg-gray-800 rounded-lg p-6 space-y-4">
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <span className="text-gray-400 text-sm">Status</span>
                      <p className="text-green-400 font-medium">Powered Off</p>
                    </div>
                    <div>
                      <span className="text-gray-400 text-sm">OS Type</span>
                      <p>{currentVM.osType}</p>
                    </div>
                    <div>
                      <span className="text-gray-400 text-sm">CPU Cores</span>
                      <p>{currentVM.cpuCores}</p>
                    </div>
                    <div>
                      <span className="text-gray-400 text-sm">Memory</span>
                      <p>{currentVM.memoryMB} MB</p>
                    </div>
                    <div>
                      <span className="text-gray-400 text-sm">Disk Size</span>
                      <p>{currentVM.diskSizeGB} GB</p>
                    </div>
                    <div>
                      <span className="text-gray-400 text-sm">Boot Order</span>
                      <p>{currentVM.bootOrder === 0 ? 'CD/DVD → HDD' : 'HDD → CD/DVD'}</p>
                    </div>
                  </div>
                  <div className="flex gap-3 pt-4">
                    <button
                      onClick={() => handleStartVM(currentVM.id)}
                      className="bg-green-600 hover:bg-green-700 text-white px-6 py-2 rounded transition-colors"
                    >
                      ▶ Start
                    </button>
                    <button
                      onClick={() => setShowSettings(true)}
                      className="bg-gray-700 hover:bg-gray-600 text-white px-6 py-2 rounded transition-colors"
                    >
                      ⚙ Settings
                    </button>
                  </div>
                </div>
              </div>
            </div>
          ) : (
            <div className="flex-1 flex items-center justify-center">
              <div className="text-center text-gray-500">
                <div className="text-6xl mb-4">🖥️</div>
                <p className="text-xl mb-2">Welcome to WebVirtualBox</p>
                <p className="text-sm">Create a new virtual machine to get started</p>
                <button
                  onClick={() => setShowCreateDialog(true)}
                  className="mt-4 bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded transition-colors"
                >
                  Create Virtual Machine
                </button>
              </div>
            </div>
          )}
        </main>
      </div>

      {/* Dialogs */}
      {showCreateDialog && (
        <VMCreateDialog
          onClose={() => setShowCreateDialog(false)}
          onCreate={handleCreateVM}
        />
      )}
      
      {showSettings && currentVM && (
        <VMSettings
          vm={currentVM}
          onClose={() => setShowSettings(false)}
          onSave={(config: VMConfig) => {
            const storage = new VMStorage();
            storage.saveVM(config);
            setVMs(vms.map(vm => vm.id === config.id ? config : vm));
            setShowSettings(false);
          }}
        />
      )}
    </div>
  );
}
