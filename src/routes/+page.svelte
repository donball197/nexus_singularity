<script>
  import { onMount, tick } from 'svelte';
  
  let files=[], currentFile="", currentPath="", editorContent="", status="Ready", isSidebarOpen=true;
  let termOutput=">> COMMANDER GEMMA-3 ONLINE\n>> UPLINK ESTABLISHED.\n", termCmd="";
  let termElement;

  // --- FILE SYSTEM ---
  async function fetchFiles() { try { const res = await fetch('/api/files'); files = await res.json(); } catch(e){} }
  
  async function openFile(f) { 
    if(f.is_dir) return; 
    currentPath=f.path; currentFile=f.name; status="Reading...";
    try {
        const res = await fetch(`/api/read?path=${encodeURIComponent(f.path)}`);
        editorContent = await res.text(); status="Ready";
    } catch(e) { status="Error"; }
  }

  async function saveFile() {
    if (!currentPath) return;
    status="Saving...";
    try {
        await fetch('/api/save', { method:'POST', body:JSON.stringify({path:currentPath, content:editorContent}) });
        status="Saved"; setTimeout(()=>status="Ready", 2000);
    } catch(e) { status="Error"; }
  }

  // --- THE BRAIN INTERFACE ---
  async function runCmd() {
    if(!termCmd) return;
    let raw = termCmd; 
    termCmd="";
    
    // Display what the user typed
    termOutput += `> ${raw}\n`;
    termOutput += ">> [COMMANDER] Processing...\n";
    await scrollToBottom();

    // --- FIX: CLEAN THE INPUT ---
    // Remove "nexus:" prefix so Gemma doesn't think 'nexus' is an agent name
    const cleanText = raw.replace(/^nexus:\s*/i, "").trim();

    try {
        const res = await fetch('/api/ask', {
            method:'POST', headers:{'Content-Type':'application/json'}, 
            body:JSON.stringify({text:cleanText}) // Send clean text
        });
        const json = await res.json();
        
        if(json.candidates && json.candidates[0].content) {
            const answer = json.candidates[0].content.parts[0].text;
            termOutput += `>> ${answer}\n`;
        } else {
            termOutput += `>> [ERR] PROTOCOL MISMATCH: ${JSON.stringify(json)}\n`;
        }
    } catch(e) { 
        termOutput += `>> [ERR] UPLINK FAILED: ${e}\n`; 
    }
    
    // Refresh files in case Gemma created an agent
    await fetchFiles();
    await scrollToBottom();
  }

  async function scrollToBottom() {
      await tick();
      if(termElement) termElement.scrollTop = termElement.scrollHeight;
  }
  
  function handleKey(e) { if(e.key==='Enter') runCmd(); }
  onMount(fetchFiles);
</script>

<main style="display:flex; height:100vh; background:#050505; color:#c0c0c0; font-family:sans-serif; overflow:hidden;">
  <!-- SIDEBAR -->
  <div style="width:{isSidebarOpen?'250px':'0px'}; transition:width 0.3s; border-right:1px solid #222; background:#0a0a0a; overflow-y:auto; display:flex; flex-direction:column;">
    <div style="padding:15px;">
      <h3 style="color:#00ff9d; font-size:12px; letter-spacing:2px; margin-bottom:15px;">NEXUS FORGE</h3>
      {#each files as f} 
        <div on:click={()=>openFile(f)} style="cursor:pointer; padding:8px 0; font-size:13px; color:{f.is_dir?'#e6db74':'#888'}; border-bottom:1px solid #1a1a1a; display:flex; align-items:center;">
          <span style="margin-right:8px; font-size:10px; opacity:0.5;">{f.is_dir?'DIR':'DOC'}</span> {f.name}
        </div> 
      {/each}
    </div>
  </div>

  <!-- MAIN AREA -->
  <div style="flex:1; display:flex; flex-direction:column;">
    <!-- TOOLBAR -->
    <div style="padding:10px 20px; background:#0f0f0f; border-bottom:1px solid #222; display:flex; justify-content:space-between; align-items:center;">
      <div style="display:flex; align-items:center; gap:15px;">
        <button on:click={()=>isSidebarOpen=!isSidebarOpen} style="background:none; border:none; color:#555; cursor:pointer; font-size:16px;">☰</button>
        <span style="font-size:12px; color:#555;">{currentFile || 'Awaiting Input'} <span style="color:#333; margin:0 5px;">|</span> <span style="color:#00ff9d">{status}</span></span>
      </div>
      <button on:click={saveFile} style="background:#00ff9d; border:none; color:black; font-weight:bold; font-size:11px; padding:6px 12px; border-radius:2px; cursor:pointer; letter-spacing:1px;">SAVE</button>
    </div>

    <!-- EDITOR -->
    <textarea bind:value={editorContent} spellcheck="false" style="flex:1; background:#050505; color:#d4d4d4; border:none; padding:20px; font-family:'Courier New', monospace; font-size:14px; outline:none; resize:none; line-height:1.6;"></textarea>

    <!-- TERMINAL -->
    <div style="height:250px; background:#080808; border-top:1px solid #222; display:flex; flex-direction:column;">
        <div bind:this={termElement} id="term" style="flex:1; padding:15px; overflow-y:auto; font-family:'Courier New', monospace; font-size:13px; color:#00ff9d; white-space:pre-wrap; line-height:1.4;">{termOutput}</div>
        <div style="display:flex; padding:10px; background:#0f0f0f; align-items:center;">
            <span style="padding-right:10px; color:#00ff9d; font-weight:bold;">$</span>
            <input bind:value={termCmd} on:keydown={handleKey} type="text" style="flex:1; background:transparent; border:none; color:white; font-family:'Courier New', monospace; font-size:13px; outline:none;" placeholder="Talk to Commander Gemma..." />
        </div>
    </div>
  </div>
</main>
