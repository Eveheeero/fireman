import React, { useState, createContext, useMemo } from 'react';
import * as rs from "./bindings";

interface ContextData {
  knownSections: rs.KnownSection[];
  setKnownSections: React.Dispatch<React.SetStateAction<rs.KnownSection[]>>;
}
export const Context = createContext<ContextData>({
  knownSections: [],
  setKnownSections: () => { }
});

export const ContextProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [knownSections, setKnownSections] = useState<rs.KnownSection[]>([]);

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
