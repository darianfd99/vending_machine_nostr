import React from 'react';

const ChangePriceForm = ({ 
  selectedItem, 
  setSelectedItem, 
  newPrice, 
  setNewPrice,
  handleChangePrice 
}) => {
  return (
    <div className="bg-gray-50 p-4 rounded-lg">
      <h3 className="text-md font-medium text-gray-900 mb-4">Change Price</h3>
      {selectedItem ? (
        <div className="space-y-4">
          <div>
            <p className="text-sm text-gray-700 mb-2">
              Selected Item: <span className="font-medium">{selectedItem.name}</span> (ID: {selectedItem.id})
            </p>
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              New Price
            </label>
            <input
              type="number"
              value={newPrice}
              onChange={(e) => setNewPrice(e.target.value)}
              min="1"
              className="w-full p-2 border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-blue-500"
              placeholder="New price"
            />
          </div>
          
          <div className="flex space-x-2">
            <button
              onClick={handleChangePrice}
              className="flex-1 bg-blue-600 text-white py-2 px-4 rounded hover:bg-blue-700 transition duration-200"
            >
              Update Price
            </button>
            <button
              onClick={() => {
                setSelectedItem(null);
                setNewPrice('');
              }}
              className="flex-1 bg-gray-200 text-gray-800 py-2 px-4 rounded hover:bg-gray-300 transition duration-200"
            >
              Cancel
            </button>
          </div>
        </div>
      ) : (
        <div className="text-center py-8 text-gray-500">
          <p>Select an item from the inventory to change its price</p>
        </div>
      )}
    </div>
  );
};

export default ChangePriceForm;