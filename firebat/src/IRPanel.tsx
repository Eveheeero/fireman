import { useContext } from "react";
import { Context } from "./context";

function IRPanel() {
  const { decompileResult, hoveredAssemblyIndex, setHoveredAssemblyIndex } = useContext(Context);

  return (
    <div className="flex flex-col h-full">
      <div className="p-4 flex-shrink-0">
        <h2 className="text-lg font-bold">IR Panel</h2>
      </div>
      <div className="flex-1 overflow-auto px-4 pb-4">
        {decompileResult?.data.ir && decompileResult.data.ir.length > 0 ? (
          <div className="space-y-2">
            {decompileResult.data.ir.map((irItem, index) => {
              const isHovered = hoveredAssemblyIndex === irItem.parentsAssemblyIndex;
              const baseColor = decompileResult.colors.get(irItem.parentsAssemblyIndex) ?? 'bg-gray-100';
              const hoverColor = isHovered ? 'ring-2 ring-blue-500 shadow-md' : '';

              return (
                <pre
                  key={index}
                  className={`${baseColor} ${hoverColor} p-3 rounded border text-sm font-mono whitespace-pre-wrap overflow-auto text-left select-text cursor-pointer transition-all`}
                  onMouseEnter={() => setHoveredAssemblyIndex(irItem.parentsAssemblyIndex)}
                  onMouseLeave={() => setHoveredAssemblyIndex(null)}
                >
                  {irItem.data}
                </pre>
              );
            })}
          </div>
        ) : (
          <p className="text-gray-500">No IR data available. Please select sections and run decompilation.</p>
        )}
      </div>
    </div>
  );
}

export default IRPanel;