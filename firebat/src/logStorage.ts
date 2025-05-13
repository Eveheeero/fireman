import { create } from 'zustand';

interface LogState {
  logs: string[];
  addLog: (message: string) => void;
}

const logStorage = create<LogState>((set) => ({
  logs: [],
  addLog: (message) =>
    set((state) => {
      return { logs: [...state.logs, message] };
    }),
}));

export default logStorage;