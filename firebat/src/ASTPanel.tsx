import { useContext } from "react";
import { Context } from "./context";

function ASTPanel() {
  const { decompileResult } = useContext(Context);

  return (
    <div className="flex flex-col h-full">
      <div className="flex-1 overflow-auto">
        <div className="p-4">
          <h2 className="text-lg font-bold">AST Panel</h2>
          {decompileResult?.decompiled ? (
            <div className="mt-4">
              <h3 className="text-md font-semibold mb-2">Decompiled Code:</h3>
              <pre className="bg-gray-100 p-3 rounded border text-sm font-mono whitespace-pre-wrap overflow-auto text-left select-text">
                {decompileResult.decompiled}
              </pre>
            </div>
          ) : (
            <p className="mt-4 text-gray-500">No decompiled code available. Please select sections and run decompilation.</p>
          )}
        </div>
      </div>
    </div>
  );
}

export default ASTPanel;