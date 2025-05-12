import React, { useState } from 'react';
import logStorage from './logStorage';

const LogBar: React.FC = () => {
  const [isExpanded, setIsExpanded] = useState(false);
  const logs = logStorage(state => state.logs);
  const latestLog = logs[logs.length - 1];
  const displayMessage = latestLog ?? 'Log Shows Here';

  const handleClick = () => {
    setIsExpanded(!isExpanded);
  };

  return (
    <div
      className={`
        fixed bottom-0 left-0 right-0 bg-gray-900 text-white p-2 text-sm cursor-pointer
        transition-all duration-300 ease-in-out
        ${isExpanded ? 'h-60 overflow-y-auto' : 'h-8 overflow-hidden'}
      `}
      onClick={handleClick}
    >
      {isExpanded ? (
        logs.length > 0 ? (
          <div className="flex flex-col-reverse">
            {logs.map((logMessage, index) => (
              <div key={index} className="py-0.5 border-b border-gray-700 first:border-b-0">
                {logMessage}
              </div>
            ))}
          </div>
        ) : (
          <div>{displayMessage}</div>
        )
      ) : (
        <div className="truncate">
          {displayMessage}
        </div>
      )}
    </div>
  );
};

export default LogBar;