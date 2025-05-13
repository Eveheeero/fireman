import logStorage from './logStorage';

const formatLogMessage = (message: string): string => {
  const now = new Date();
  const timestamp = now.toLocaleTimeString();
  return `[${timestamp}] ${message}`;
};

export const log = (...data: any[]) => {
  let s = "";
  for (const i of data) {
    if (typeof i === "object") {
      s += JSON.stringify(i);
    } else {
      s += i;
    }
    s += " ";
  }
  const formattedMessage = formatLogMessage(s);
  logStorage.getState().addLog(formattedMessage);
};
