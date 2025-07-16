import React, { useState } from 'react';

interface ErrorTesterProps {
  onTriggerError: () => void;
}

export const ErrorTester: React.FC<ErrorTesterProps> = ({ onTriggerError }) => {
  const [shouldError, setShouldError] = useState(false);

  if (shouldError) {
    // This will trigger an error that the ErrorBoundary should catch
    throw new Error('Test error triggered by ErrorTester component');
  }

  return (
    <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
      <h3 className="text-sm font-semibold text-yellow-800 mb-2">Error Boundary Test</h3>
      <p className="text-xs text-yellow-700 mb-3">
        This component is for testing the error boundary. Click the button to trigger an intentional error.
      </p>
      <button
        onClick={() => setShouldError(true)}
        className="bg-red-500 text-white px-3 py-1 rounded text-xs hover:bg-red-600"
      >
        Trigger Test Error
      </button>
    </div>
  );
};