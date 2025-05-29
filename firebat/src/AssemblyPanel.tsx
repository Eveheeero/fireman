import { useContext } from "react";
import { Context } from "./context";

function AssemblyPanel() {
  const { decompileResult } = useContext(Context);
  
  return (
    <div className="flex flex-col h-full">
      <div className="flex-1 overflow-auto">
        <div className="p-4">
          <h2 className="text-lg font-bold">Assembly Panel</h2>
          {decompileResult?.data.assembly && decompileResult.data.assembly.length > 0 ? (
            <div className="mt-4 space-y-2">
              {decompileResult.data.assembly.map((assemblyItem, index) => (
                <pre key={index} className={`${decompileResult.colors.get(assemblyItem.index) || 'bg-gray-100'} p-3 rounded border text-sm font-mono whitespace-pre-wrap overflow-auto text-left select-text`}>
                  {assemblyItem.data}
                </pre>
              ))}
            </div>
          ) : (
            <p className="mt-4 text-gray-500">No assembly data available. Please select sections and run decompilation.</p>
          )}
        </div>
      </div>
    </div>
  );
}

export default AssemblyPanel;