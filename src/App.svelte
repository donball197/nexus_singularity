<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  let response = "Awaiting Commander Pulse...";
  let loading = false;

  async function handshake() {
    loading = true;
    try {
      response = await invoke("commander_handshake");
    } catch (e) {
      response = `Nerve Failure: ${e}`;
    } finally {
      loading = false;
    }
  }
</script>

<main class="container">
  <div class="header">
    <h1>PROJECT NEXUS SINGULARITY</h1>
    <p class="status-tag">NODE: ORACLE-DUET-DEBIAN | v0.3.9</p>
  </div>

  <div class="tactical-grid">
    <div class="card">
      <button on:click={handshake} disabled={loading} class="pulse-btn">
        {loading ? "SYNCHRONIZING..." : "IGNITE COMMANDER HANDSHAKE"}
      </button>
      <div class="terminal-box">
        <pre>{response}</pre>
      </div>
    </div>
  </div>
</main>

<style>
  .container { background-color: #0d0d0d; color: #00ff41; min-height: 100vh; display: flex; flex-direction: column; align-items: center; font-family: 'Courier New', Courier, monospace; }
  .header { margin-top: 5vh; text-align: center; border-bottom: 1px solid #00ff41; width: 80%; }
  .status-tag { font-size: 0.8rem; color: #ffaa00; }
  .tactical-grid { margin-top: 2rem; width: 90%; max-width: 800px; }
  .card { background: #1a1a1a; padding: 1.5rem; border: 1px solid #333; border-radius: 4px; }
  .pulse-btn { width: 100%; padding: 1rem; background: #004400; color: #00ff41; border: 1px solid #00ff41; cursor: pointer; font-weight: bold; }
  .pulse-btn:disabled { background: #222; color: #555; border-color: #333; }
  .terminal-box { margin-top: 1rem; background: #000; border: 1px dashed #00ff41; padding: 0.5rem; height: 200px; overflow-y: auto; }
  pre { white-space: pre-wrap; word-wrap: break-word; font-size: 0.9rem; }
</style>
