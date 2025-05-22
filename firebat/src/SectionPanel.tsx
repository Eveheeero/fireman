import { invoke } from "@tauri-apps/api/core";
import { SetStateAction, useState } from "react";
import { log } from "./logger";
import * as rs from "./bindings";

function SectionPanel() {
  const [decompileResult, setDecompileResult] = useState<string>("");
  const [decompileTargetAddress, setDecompileTargetAddress] = useState<string>("");
  const [inspectResult, setInspectResult] = useState<rs.IrInspectResult[]>([]);

  async function decomFromAddress() {
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


  return (
    <div className="flex flex-col h-full">
      <div className="flex-1 overflow-auto">
        <div className="p-4">
          <h2 className="text-lg font-bold">Section Panel</h2>
          <p>Section panel content goes here.</p>
        </div>
      </div>
    </div>
  );
}

export default SectionPanel;