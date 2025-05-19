import React from 'react';
import { Plus } from 'lucide-react';

const AddItemForm = ({ 
  newItemId, 
  setNewItemId, 
  newItemName, 
  setNewItemName, 
  newItemPrice, 
  setNewItemPrice, 
  newItemCount, 
  setNewItemCount, 
  handleAddItem, 
  items 
}) => {
  const isExistingItem = items.some(item => item.id === parseInt(newItemId));
  
  return (
    <div className="bg-gray-50 p-4 rounded-lg">
      <h3 className="text-md font-medium text-gray-900 mb-4">Add or Restock Item</h3>
      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Item ID
          </label>
          <input
            type="number"
            value={newItemId}
            onChange={(e) => setNewItemId(e.target.value)}
            min="1"
            className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-blue-500"
            placeholder="Item ID"
          />
        </div>
        
        {!isExistingItem && (
          <>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Name
              </label>
              <input
                type="text"
                value={newItemName}
                onChange={(e) => setNewItemName(e.target.value)}
                className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-blue-500"
                placeholder="Item name"
              />
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Price
              </label>
              <input
                type="number"
                value={newItemPrice}
                onChange={(e) => setNewItemPrice(e.target.value)}
                min="1"
                className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-blue-500"
                placeholder="Price"
              />
            </div>
          </>
        )}
        
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Quantity to add
          </label>
          <input
            type="number"
            value={newItemCount}
            onChange={(e) => setNewItemCount(e.target.value)}
            min="1"
            className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-blue-500"
            placeholder="Quantity"
          />
        </div>
        
        <button
          onClick={handleAddItem}
          className="w-full bg-blue-600 text-white py-2 px-4 rounded hover:bg-blue-700 transition duration-200 flex items-center justify-center"
        >
          <Plus size={16} className="mr-2" />
          {isExistingItem ? 'Restock Item' : 'Add New Item'}
        </button>
      </div>
    </div>
  );
};

export default AddItemForm;