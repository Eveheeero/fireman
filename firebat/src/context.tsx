import React, { useState, createContext, useMemo } from 'react';
import * as rs from "./bindings";

export interface KnownSection {
  selected: boolean;
  data: rs.KnownSection;
}

export interface DecompileResult {
  colors: Map<number, string>;
  data: rs.DecompileResult;
}

export const getColorForIndex = (index: number): string => {
  const colors = [
    'bg-red-900', 'bg-blue-900', 'bg-green-900', 'bg-yellow-800',
    'bg-purple-900', 'bg-pink-900', 'bg-indigo-900', 'bg-orange-900',
    'bg-teal-900', 'bg-cyan-900'
  ];
  return colors[index % colors.length];
};

interface ContextData {
  knownSections: KnownSection[];
  setKnownSections: React.Dispatch<React.SetStateAction<KnownSection[]>>;
  decompileResult: DecompileResult | null;
  setDecompileResult: React.Dispatch<React.SetStateAction<DecompileResult | null>>;
  hoveredAssemblyIndex: number | null;
  setHoveredAssemblyIndex: React.Dispatch<React.SetStateAction<number | null>>;
}
export const Context = createContext<ContextData>({
  knownSections: [],
  setKnownSections: () => { },
  decompileResult: null,
  setDecompileResult: () => { },
  hoveredAssemblyIndex: null,
  setHoveredAssemblyIndex: () => { },
});

export const ContextProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [knownSections, setKnownSections] = useState<KnownSection[]>([]);
  const [decompileResult, setDecompileResult] = useState<DecompileResult | null>(null);
  const [hoveredAssemblyIndex, setHoveredAssemblyIndex] = useState<number | null>(null);

  const obj = useMemo(() => ({
    knownSections,
    setKnownSections,
    decompileResult,
    setDecompileResult,
    hoveredAssemblyIndex,
    setHoveredAssemblyIndex
  }), [knownSections, setKnownSections, decompileResult, setDecompileResult, hoveredAssemblyIndex, setHoveredAssemblyIndex]);
  return (
    <Context.Provider value={obj}>
      {children}
    </Context.Provider>
  );
}
