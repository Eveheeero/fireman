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
    'bg-red-100', 'bg-blue-100', 'bg-green-100', 'bg-yellow-100', 
    'bg-purple-100', 'bg-pink-100', 'bg-indigo-100', 'bg-orange-100',
    'bg-teal-100', 'bg-cyan-100'
  ];
  return colors[index % colors.length];
};

interface ContextData {
  knownSections: KnownSection[];
  setKnownSections: React.Dispatch<React.SetStateAction<KnownSection[]>>;
  decompileResult: DecompileResult | null;
  setDecompileResult: React.Dispatch<React.SetStateAction<DecompileResult | null>>;
}
export const Context = createContext<ContextData>({
  knownSections: [],
  setKnownSections: () => { },
  decompileResult: null,
  setDecompileResult: () => { },
});

export const ContextProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [knownSections, setKnownSections] = useState<KnownSection[]>([]);
  const [decompileResult, setDecompileResult] = useState<DecompileResult | null>(null);

  const obj = useMemo(() => ({
    knownSections,
    setKnownSections,
    decompileResult,
    setDecompileResult
  }), [knownSections, setKnownSections, decompileResult, setDecompileResult]);
  return (
    <Context.Provider value={obj}>
      {children}
    </Context.Provider>
  );
}
