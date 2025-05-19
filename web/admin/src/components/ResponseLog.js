import React from 'react';

const ResponseLog = ({ responses }) => {
  if (responses.length === 0) {
    return (
      <div className="bg-gray-50 p-4 rounded-lg">
        <h3 className="text-md font-medium text-gray-900 mb-4">Machine Responses</h3>
        <p className="text-center text-gray-500 py-4">No responses yet</p>
      </div>
    );
  }
  
  return (
    <div className="bg-gray-50 p-4 rounded-lg">
      <h3 className="text-md font-medium text-gray-900 mb-4">Machine Responses</h3>
      <div className="overflow-y-auto max-h-64 border border-gray-200 rounded">
        {responses.map(response => (
          <div key={response.id} className="p-3 border-b border-gray-200 last:border-b-0">
            <div className="flex justify-between text-xs text-gray-500 mb-1">
              <span>{response.timestamp}</span>
              <span>{response.content.type}</span>
            </div>
            <pre className="text-xs bg-gray-100 p-2 rounded overflow-x-auto">
              {JSON.stringify(response.content, null, 2)}
            </pre>
          </div>
        ))}
      </div>
    </div>
  );
};

export default ResponseLog;