import React from 'react';
import { PenLine, Trash2 } from 'lucide-react';
import AddItemForm from './AddItemForm';
import ChangePriceForm from './ChangePriceForm';

const InventoryTab = ({ 
  items = [],
  newItemId,
  setNewItemId,
  newItemName,
  setNewItemName,
  newItemPrice,
  setNewItemPrice,
  newItemCount,
  setNewItemCount,
  selectedItem,
  setSelectedItem,
  newPrice,
  setNewPrice,
  handleAddItem,
  handleRemoveItem,
  handleChangePrice
}) => {
  return (
    <div className="space-y-8">
      {/* Current Inventory */}
      <div>
        <h2 className="text-lg font-medium text-gray-900 mb-4">Current Inventory</h2>
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">ID</th>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Name</th>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Price</th>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Stock</th>
                <th scope="col" className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">Actions</th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {items.length > 0 ? (
                items.map((item) => (
                  <tr key={item.id}>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{item.id}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{item.name}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{item.price}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">{item.count}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                      <button
                        onClick={() => {
                          setSelectedItem(item);
                          setNewPrice(item.price.toString());
                        }}
                        className="text-blue-600 hover:text-blue-900 mr-3"
                      >
                        <PenLine size={18} />
                      </button>
                      <button
                        onClick={() => handleRemoveItem(item.id)}
                        className="text-red-600 hover:text-red-900"
                      >
                        <Trash2 size={18} />
                      </button>
                    </td>
                  </tr>
                ))
              ) : (
                <tr>
                  <td colSpan="5" className="px-6 py-4 text-center text-sm text-gray-500">No items in inventory</td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* Forms Section - 2 column layout */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <AddItemForm
          newItemId={newItemId}
          setNewItemId={setNewItemId}
          newItemName={newItemName}
          setNewItemName={setNewItemName}
          newItemPrice={newItemPrice}
          setNewItemPrice={setNewItemPrice}
          newItemCount={newItemCount}
          setNewItemCount={setNewItemCount}
          handleAddItem={handleAddItem}
          items={items}
        />
        
        <ChangePriceForm
          selectedItem={selectedItem}
          setSelectedItem={setSelectedItem}
          newPrice={newPrice}
          setNewPrice={setNewPrice}
          handleChangePrice={handleChangePrice}
        />
      </div>
    </div>
  );
};

export default InventoryTab;