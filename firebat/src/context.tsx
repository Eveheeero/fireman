import React, { useState, createContext, useMemo } from 'react';
import * as rs from "./bindings";

export interface KnownSection {
  selected: boolean;
  data: rs.KnownSection;
}

interface ContextData {
  knownSections: KnownSection[];
  setKnownSections: React.Dispatch<React.SetStateAction<KnownSection[]>>;
  decompileResult: rs.DecompileResult | null;
  setDecompileResult: React.Dispatch<React.SetStateAction<rs.DecompileResult | null>>;
}
export const Context = createContext<ContextData>({
  knownSections: [],
  setKnownSections: () => { },
  decompileResult: null,
  setDecompileResult: () => { },
});

export const ContextProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [knownSections, setKnownSections] = useState<KnownSection[]>([]);
  const [decompileResult, setDecompileResult] = useState<rs.DecompileResult | null>(null);

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
