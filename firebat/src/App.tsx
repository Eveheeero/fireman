import { useState } from "react";
import logoBackgrounded from "./assets/logo colored 512.png";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Navigation from "./Navigation";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main>
      <Navigation />
      <h1>Welcome to Tauri + React</h1>

      <div className="container">
        {/* <div className="row invisible">
        <img src="/logo transparent.svg" className="logo" alt="transparent logo" />
        <img src={logoBackgrounded} className="logo" alt="backgrounded logo" />
      </div> */}
        <p>Click on the Tauri, Vite, and React logos to learn more.</p>

        <form
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <input
            onChange={(e) => setName(e.currentTarget.value)}
            autoComplete="off"
            placeholder="Enter a name..."
          />
          <button type="submit">Greet</button>
        </form>
        <p>{greetMsg}</p>
      </div>
    </main>
  );
}

export default App;
