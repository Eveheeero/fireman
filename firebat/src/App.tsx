import { SetStateAction, useState } from "react";
import logoBackgrounded from "./assets/logo colored 512.png";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Navigation from "./Navigation";
import LogBar from "./LogBar";
import { log } from "./logger";
import * as rs from "./bindings";
import GraphView from "./GraphView";

function App() {
  const [decompileResult, setDecompileResult] = useState<string>("");
  const [decompileTargetAddress, setDecompileTargetAddress] = useState<string>("");
  const [inspectResult, setInspectResult] = useState<rs.IrInspectResult[]>([]);

  async function greet() {
    await invoke("decom_from_address", { address: decompileTargetAddress }).then((result) => {
      log("Decompile Success", result);
      setDecompileResult("Block's Connected Address : " + result as SetStateAction<string>);
    }).catch((error) => {
      log("Decompile Failed", error);
    });
  }

  async function decomFromEntry() {
    await invoke("decom_from_entry").then((result) => {
      log("Decompile Success", result);
      setDecompileResult("Block's Connected Address : " + result as SetStateAction<string>);
    }).catch((error) => {
      log("Decompile Failed", error);
    });
  }

  async function inspect() {
    await invoke<rs.IrInspectResult[]>("ir_inspect", { address: decompileTargetAddress }).then((result) => {
      log("Inspect Success", result);
      setInspectResult(result);
    }).catch((error) => {
      log("Inspect Failed", error);
    });
  }

  return (
    <main>
      <Navigation />
      {/* <div className="row">
        <img src="/logo transparent.svg" className="logo" alt="transparent logo" />
        <img src={logoBackgrounded} className="logo" alt="backgrounded logo" />
      </div> */}
      <input
        onChange={(e) => setDecompileTargetAddress(e.currentTarget.value)}
        autoComplete="off"
        placeholder="Enter Position to Decompile"
      />
      <button onClick={greet} className="dft-btn">Decompile From Address</button>
      <button onClick={inspect} className="dft-btn">Inspect Address</button>
      <button onClick={decomFromEntry} className="dft-btn">decom from entry</button>
      <p>{decompileResult}</p>

      {inspectResult.length > 0 && <GraphView data={inspectResult} />}

      <LogBar />
    </main>
  );
}

export default App;
