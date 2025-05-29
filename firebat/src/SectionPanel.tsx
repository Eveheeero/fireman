import { invoke } from "@tauri-apps/api/core";
import { useContext, useState } from "react";
import { log } from "./logger";
import * as rs from "./bindings";
import { Context, getColorForIndex } from "./context";

function SectionPanel() {
  const [analyzeTargetAddress, setAnalyzeTargetAddress] = useState<string>("");
  const { knownSections, setKnownSections, setDecompileResult } = useContext(Context);

  async function analyzeSectionFromAddress(startAddress: string) {
    if (knownSections.some(section => section.data.analyzed && section.data.startAddress === Number(startAddress))) {
      log("Section already known", startAddress);
      return;
    }
    await invoke("analyze_section", { address: startAddress }).then((result) => {
      log("Section Analyzation Success", result);
      const newSections = result as rs.KnownSection[];
      setKnownSections(prev => [
        ...prev.filter(section => !newSections.some(newSection => newSection.startAddress === section.data.startAddress)),
        ...newSections.map(section => ({ selected: false, data: section }))
      ]);
    }).catch((error) => {
      log("Section Analyzation Failed", error);
    });
  }

  function selectAll() {
    const analyzedSections = knownSections.filter(section => section.data.analyzed);
    const allSelected = analyzedSections.every(section => section.selected);

    setKnownSections(prev => prev.map(section =>
      section.data.analyzed
        ? { ...section, selected: !allSelected }
        : section
    ));
  }

  async function decompileSelected() {
    const selectedSections = knownSections.filter(section => section.selected);
    if (selectedSections.length === 0) {
      log("No sections selected for decompilation");
      return;
    }

    // generate list of startAddress
    let startAddresses = selectedSections.map(section => section.data.startAddress);
    log("Decompiling sections", startAddresses);
    await invoke("decompile_sections", { start_addresses: startAddresses }).then((result) => {
      const decompiledResult = result as rs.DecompileResult;

      // 색상 맵 생성
      const colorMap = new Map<number, string>();
      decompiledResult.assembly.forEach(assembly => {
        colorMap.set(assembly.index, getColorForIndex(assembly.index));
      });

      setDecompileResult({ colors: colorMap, data: decompiledResult });
      log("Decompilation Success");
    }).catch((error) => {
      log("Decompilation Failed", error);
    });
  }

  return (
    <div className="flex flex-col h-full">
      <div className="flex flex-col gap-1 p-2 border-b">
        <div className="flex items-center gap-1">
          <input
            type="text"
            value={analyzeTargetAddress}
            onChange={(e) => setAnalyzeTargetAddress(e.target.value)}
            placeholder="Empty if you want to decompile from entry"
            className="flex-1 px-2 py-1 text-sm border border-gray-300 rounded focus:outline-none focus:ring-1 focus:ring-blue-500"
          />
          <button
            onClick={(_e) => analyzeSectionFromAddress(analyzeTargetAddress)}
            className="dft-btn"
          >
            Analyze Address
          </button>
        </div>
        <div className="flex items-center gap-1">
          <button
            onClick={selectAll}
            className="dft-btn flex-1"
          >
            Select All
          </button>
          <button
            onClick={decompileSelected}
            className="dft-btn flex-1"
          >
            Decompile Selected
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-auto">
        <div className="p-4">
          <h2 className="text-lg font-bold">Known Sections</h2>
          <ul className="mt-2 space-y-2">
            {knownSections.map((section, index) => (
              <li key={index} className="p-2 border rounded bg-gray-500">
                <div className="flex justify-between items-start">
                  <p className="flex items-center">
                    <input
                      type="checkbox"
                      checked={section.selected}
                      onChange={() => {
                        setKnownSections(prev => prev.map((s, i) => i === index ? { ...s, selected: !s.selected } : s));
                      }}
                      disabled={!section.data.analyzed}
                      className="mr-2"
                    />
                    <p>
                      0x{section.data.startAddress.toString(16)}..{section.data.endAddress?.toString(16)}
                    </p>
                  </p>
                  <span>
                    {section.data.analyzed ? "Analyzed" : (
                      <button
                        onClick={() => analyzeSectionFromAddress(section.data.startAddress.toString())}
                        className="dft-btn"
                      >
                        Analyze Section
                      </button>
                    )}
                  </span>
                </div>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}

export default SectionPanel;