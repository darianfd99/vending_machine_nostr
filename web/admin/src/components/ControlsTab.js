import React from 'react';
import { Database, RefreshCw, Power, Terminal, LogOut } from 'lucide-react';

const ControlsTab = ({ 
  handleRequestStatus, 
  handleReboot, 
  handleShutdown, 
  handleEnterAdminMode, 
  handleEndAdminMode,
  isConnected
}) => {
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h2 className="text-lg font-medium text-gray-900">Machine Controls</h2>
        <div className="flex items-center">
          <span 
            className={`inline-block w-3 h-3 rounded-full mr-2 ${isConnected ? 'bg-green-500' : 'bg-red-500'}`} 
            title={isConnected ? 'Connected to Nostr network' : 'Disconnected from Nostr network'}
          ></span>
          <span className="text-sm text-gray-600">{isConnected ? 'Connected' : 'Disconnected'}</span>
        </div>
      </div>
      
      {/* Control buttons */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div className="bg-gray-50 p-4 rounded-lg text-center">
          <button
            onClick={handleRequestStatus}
            className="w-full bg-blue-600 text-white py-3 px-4 rounded hover:bg-blue-700 transition duration-200 flex items-center justify-center"
          >
            <Database size={20} className="mr-2" />
            Request Status
          </button>
          <p className="mt-2 text-xs text-gray-500">Get current machine status and inventory</p>
        </div>
        
        <div className="bg-gray-50 p-4 rounded-lg text-center">
          <button
            onClick={handleReboot}
            className="w-full bg-amber-500 text-white py-3 px-4 rounded hover:bg-amber-600 transition duration-200 flex items-center justify-center"
          >
            <RefreshCw size={20} className="mr-2" />
            Reboot Machine
          </button>
          <p className="mt-2 text-xs text-gray-500">Restart the vending machine</p>
        </div>
        
        <div className="bg-gray-50 p-4 rounded-lg text-center">
          <button
            onClick={handleShutdown}
            className="w-full bg-red-600 text-white py-3 px-4 rounded hover:bg-red-700 transition duration-200 flex items-center justify-center"
          >
            <Power size={20} className="mr-2" />
            Shutdown
          </button>
          <p className="mt-2 text-xs text-gray-500">Power off the vending machine</p>
        </div>
      </div>
      
      <div className="mt-8 grid grid-cols-1 md:grid-cols-2 gap-4">
        <div className="bg-gray-50 p-4 rounded-lg text-center">
          <button
            onClick={handleEnterAdminMode}
            className="w-full bg-green-600 text-white py-3 px-4 rounded hover:bg-green-700 transition duration-200 flex items-center justify-center"
          >
            <Terminal size={20} className="mr-2" />
            Enter Admin Mode
          </button>
          <p className="mt-2 text-xs text-gray-500">Request admin access on the machine</p>
        </div>
        
        <div className="bg-gray-50 p-4 rounded-lg text-center">
          <button
            onClick={handleEndAdminMode}
            className="w-full bg-gray-600 text-white py-3 px-4 rounded hover:bg-gray-700 transition duration-200 flex items-center justify-center"
          >
            <LogOut size={20} className="mr-2" />
            Exit Admin Mode
          </button>
          <p className="mt-2 text-xs text-gray-500">Exit admin mode on the machine</p>
        </div>
      </div>
      
      <div className="mt-8 p-4 bg-gray-50 rounded-lg">
        <h3 className="text-md font-medium text-gray-900 mb-2">Admin Instructions</h3>
        <div className="text-sm text-gray-600">
          <p>This interface allows you to send commands to the vending machine through the Nostr protocol.</p>
          <ul className="list-disc pl-5 mt-2 space-y-1">
            <li>Use <strong>Enter Admin Mode</strong> to first put the machine in admin mode</li>
            <li>Manage inventory in the <strong>Inventory Management</strong> tab</li>
            <li>When finished, use <strong>Exit Admin Mode</strong> to return the machine to normal operation</li>
            <li>The status indicator shows if you're connected to the Nostr network</li>
          </ul>
        </div>
      </div>
    </div>
  );
};

export default ControlsTab;