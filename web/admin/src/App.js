import React, { useState, useEffect } from 'react';
import { LogOut } from 'lucide-react';
import nostrService from './services/nostrService';
import Login from './components/Login';
import InventoryTab from './components/InventoryTab';
import ControlsTab from './components/ControlsTab';
import Notification from './components/Notification';

function App() {
  // Authentication state
  const [privateKey, setPrivateKey] = useState('');
  const [targetPubKey, setTargetPubKey] = useState('');
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [notification, setNotification] = useState(null);

  // Network state
  const [isConnected, setIsConnected] = useState(false);

  // UI state
  const [activeTab, setActiveTab] = useState('items');
  const [items, setItems] = useState([]);
  const [selectedItem, setSelectedItem] = useState(null);

  // Form state - Add Item
  const [newItemId, setNewItemId] = useState('');
  const [newItemName, setNewItemName] = useState('');
  const [newItemPrice, setNewItemPrice] = useState('');
  const [newItemCount, setNewItemCount] = useState('');

  // Form state - Change Price
  const [newPrice, setNewPrice] = useState('');

  // Load saved authentication data
  useEffect(() => {
    const savedPrivateKey = localStorage.getItem('adminPrivateKey');
    const savedTargetPubKey = localStorage.getItem('targetPubKey');
    
    if (savedPrivateKey && savedTargetPubKey) {
      setPrivateKey(savedPrivateKey);
      setTargetPubKey(savedTargetPubKey);
      setIsAuthenticated(true);
      
      // Initialize with mock data for now
      setItems([
        { id: 1, name: "Candy Bar", price: 150, count: 12 },
        { id: 2, name: "Chips", price: 200, count: 8 },
        { id: 3, name: "Soda", price: 250, count: 15 },
      ]);
      
      setIsConnected(true);
    }
  }, []);

  // Notification helper
  const showNotification = (message, type = "info") => {
    setNotification({ message, type });
    setTimeout(() => setNotification(null), 3000);
  };

  // Authentication handlers
  const handleLogin = () => {
    if (privateKey && targetPubKey) {
      try {
        setIsAuthenticated(true);
        localStorage.setItem('adminPrivateKey', privateKey);
        localStorage.setItem('targetPubKey', targetPubKey);
        showNotification("Logged in successfully", "success");
        
        // Initialize with mock data for now
        setItems([
          { id: 1, name: "Candy Bar", price: 150, count: 12 },
          { id: 2, name: "Chips", price: 200, count: 8 },
          { id: 3, name: "Soda", price: 250, count: 15 },
        ]);
        
        setIsConnected(true);
      } catch (error) {
        showNotification(`Login failed: ${error.message}`, "error");
      }
    } else {
      showNotification("Please enter both keys", "error");
    }
  };

  const handleLogout = () => {
    setIsAuthenticated(false);
    setIsConnected(false);
    setItems([]);
    setSelectedItem(null);
    
    localStorage.removeItem('adminPrivateKey');
    localStorage.removeItem('targetPubKey');
  };

  // Command sending helper
  const handleSendCommand = async (command) => {
    showNotification(`Sending ${command.type} command...`, "info");
    
    const result = await nostrService.sendCommand(privateKey, targetPubKey, command);
    
    if (result.success) {
      showNotification(result.message, "success");
      
      // Update local state to reflect changes
      if (command.type === "AddItem") {
        const newItemData = command.data;
        setItems(prevItems => {
          const existingItem = prevItems.find(item => item.id === parseInt(newItemData.id));
          if (existingItem) {
            return prevItems.map(item => 
              item.id === parseInt(newItemData.id) 
                ? {...item, count: item.count + parseInt(newItemData.count)} 
                : item
            );
          } else {
            return [...prevItems, {
              id: parseInt(newItemData.id),
              name: newItemData.name,
              price: parseInt(newItemData.price),
              count: parseInt(newItemData.count)
            }];
          }
        });
        
        // Reset form fields
        setNewItemId('');
        setNewItemName('');
        setNewItemPrice('');
        setNewItemCount('');
      } else if (command.type === "RemoveItem") {
        setItems(prevItems => prevItems.filter(item => item.id !== parseInt(command.data)));
      } else if (command.type === "ChangePrice") {
        setItems(prevItems => prevItems.map(item => 
          item.id === parseInt(command.data.id) 
            ? {...item, price: parseInt(command.data.price)} 
            : item
        ));
        
        // Reset price change state
        setSelectedItem(null);
        setNewPrice('');
      }
    } else {
      showNotification(result.message, "error");
      
      // Check if this is a connection issue
      if (result.message.includes("relay") || result.message.includes("timeout")) {
        setIsConnected(false);
        showNotification("Connection to Nostr network lost", "error");
      }
    }
  };

  // Command handlers
  const handleRequestStatus = () => {
    handleSendCommand({ type: "Status" });
  };

  const handleReboot = () => {
    if (window.confirm("Are you sure you want to reboot the vending machine?")) {
      handleSendCommand({ type: "Reboot" });
    }
  };

  const handleShutdown = () => {
    if (window.confirm("Are you sure you want to shut down the vending machine?")) {
      handleSendCommand({ type: "Shutdown" });
    }
  };
  
  const handleEnterAdminMode = () => {
    handleSendCommand({ type: "RequestAdminState" });
  };
  
  const handleEndAdminMode = () => {
    handleSendCommand({ type: "End" });
  };

  const handleAddItem = () => {
    // Validate form
    if (!newItemId || !newItemCount || (
      !items.find(item => item.id === parseInt(newItemId)) && 
      (!newItemName || !newItemPrice)
    )) {
      showNotification("Please fill all required fields", "error");
      return;
    }
    
    const command = {
      type: "AddItem",
      data: {
        id: parseInt(newItemId),
        name: newItemName || items.find(item => item.id === parseInt(newItemId))?.name || "",
        price: parseInt(newItemPrice) || items.find(item => item.id === parseInt(newItemId))?.price || 0,
        count: parseInt(newItemCount)
      }
    };
    
    handleSendCommand(command);
  };
  
  const handleRemoveItem = (id) => {
    if (window.confirm(`Are you sure you want to remove item #${id}?`)) {
      handleSendCommand({
        type: "RemoveItem",
        data: id
      });
    }
  };
  
  const handleChangePrice = () => {
    if (!selectedItem || !newPrice) {
      showNotification("Please select an item and enter a price", "error");
      return;
    }
    
    handleSendCommand({
      type: "ChangePrice",
      data: {
        id: selectedItem.id,
        price: parseInt(newPrice)
      }
    });
  };

  // Login screen
  if (!isAuthenticated) {
    return (
      <Login 
        privateKey={privateKey}
        setPrivateKey={setPrivateKey}
        targetPubKey={targetPubKey}
        setTargetPubKey={setTargetPubKey}
        handleLogin={handleLogin}
        notification={notification}
      />
    );
  }

  // Main app screen
  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <header className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex justify-between items-center">
          <div className="flex items-center">
            <h1 className="text-xl font-semibold text-gray-900">Vending Machine Admin</h1>
            <div className="ml-4 flex items-center">
              <span 
                className={`inline-block w-3 h-3 rounded-full mr-2 ${isConnected ? 'bg-green-500' : 'bg-red-500'}`} 
                title={isConnected ? 'Connected to Nostr network' : 'Disconnected from Nostr network'}
              ></span>
              <span className="text-sm text-gray-600">{isConnected ? 'Connected' : 'Disconnected'}</span>
            </div>
          </div>
          <div className="flex items-center space-x-4">
            <span className="text-sm text-gray-500">
              Connected to: {targetPubKey.substring(0, 8)}...{targetPubKey.substring(targetPubKey.length - 4)}
            </span>
            <button
              onClick={handleLogout}
              className="inline-flex items-center px-3 py-1.5 border border-transparent text-sm font-medium rounded text-gray-700 bg-gray-100 hover:bg-gray-200"
            >
              <LogOut size={16} className="mr-1" />
              Logout
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
        <div className="bg-white shadow rounded-lg overflow-hidden">
          {/* Tabs */}
          <div className="border-b border-gray-200">
            <nav className="flex -mb-px">
              <button
                onClick={() => setActiveTab('items')}
                className={`py-4 px-6 text-center border-b-2 font-medium text-sm ${
                  activeTab === 'items'
                    ? 'border-blue-500 text-blue-600'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                }`}
              >
                Inventory Management
              </button>
              <button
                onClick={() => setActiveTab('controls')}
                className={`py-4 px-6 text-center border-b-2 font-medium text-sm ${
                  activeTab === 'controls'
                    ? 'border-blue-500 text-blue-600'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                }`}
              >
                Machine Controls
              </button>
            </nav>
          </div>

          {/* Tab Content */}
          <div className="p-6">
            {activeTab === 'items' ? (
              <InventoryTab 
                items={items}
                newItemId={newItemId}
                setNewItemId={setNewItemId}
                newItemName={newItemName}
                setNewItemName={setNewItemName}
                newItemPrice={newItemPrice}
                setNewItemPrice={setNewItemPrice}
                newItemCount={newItemCount}
                setNewItemCount={setNewItemCount}
                selectedItem={selectedItem}
                setSelectedItem={setSelectedItem}
                newPrice={newPrice}
                setNewPrice={setNewPrice}
                handleAddItem={handleAddItem}
                handleRemoveItem={handleRemoveItem}
                handleChangePrice={handleChangePrice}
              />
            ) : (
              <ControlsTab 
                handleRequestStatus={handleRequestStatus}
                handleReboot={handleReboot}
                handleShutdown={handleShutdown}
                handleEnterAdminMode={handleEnterAdminMode}
                handleEndAdminMode={handleEndAdminMode}
                isConnected={isConnected}
              />
            )}
          </div>
        </div>
      </main>

      {/* Notification */}
      <Notification notification={notification} />
    </div>
  );
}

export default App;