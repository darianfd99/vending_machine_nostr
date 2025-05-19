import React from 'react';
import { AlertCircle, CheckCircle } from 'lucide-react';

const Notification = ({ notification }) => {
  if (!notification) return null;
  
  return (
    <div className={`fixed bottom-4 right-4 p-4 rounded-md shadow-md ${
      notification.type === 'error' ? 'bg-red-500' : 
      notification.type === 'success' ? 'bg-green-500' : 'bg-blue-500'
    } text-white flex items-center`}>
      {notification.type === 'error' ? <AlertCircle className="mr-2" size={20} /> : 
       notification.type === 'success' ? <CheckCircle className="mr-2" size={20} /> : null}
      {notification.message}
    </div>
  );
};

export default Notification;