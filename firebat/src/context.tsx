import React, { useState, createContext, useMemo } from 'react';
import * as rs from "./bindings";

export interface KnownSection {
  selected: boolean;
  data: rs.KnownSection;
}

interface ContextData {
  knownSections: KnownSection[];
  setKnownSections: React.Dispatch<React.SetStateAction<KnownSection[]>>;
}
export const Context = createContext<ContextData>({
  knownSections: [],
  setKnownSections: () => { }
});

export const ContextProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [knownSections, setKnownSections] = useState<KnownSection[]>([]);

  const obj = useMemo(() => ({
    knownSections,
    setKnownSections,
  }), [knownSections, setKnownSections]);
  return (
    <Context.Provider value={obj}>
      {children}
    </Context.Provider>
  );
}
