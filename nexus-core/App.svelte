<script>
  let files = [], currentFile = "", currentPath = "", editorContent = "", status = "Ready";
  async function fetchFiles() { 
    const res = await fetch('/api/files'); 
    files = await res.json(); 
  }
  async function openFile(file) {
    if (file.is_dir) return;
    currentPath = file.path; currentFile = file.name; status = "Reading...";
    const res = await fetch(`/api/read?path=${encodeURIComponent(file.path)}`);
    editorContent = await res.text(); status = "Ready";
  }
  async function saveFile() {
    status = "Saving...";
    const res = await fetch('/api/save', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({path: currentPath, content: editorContent})
    });
    status = await res.text();
  }
  fetchFiles();
</script>
<main style="display:flex; height:100vh; background:#121212; color:white; font-family:sans-serif;">
  <div style="width:220px; border-right:1px solid #333; padding:15px; overflow-y:auto; background:#181818;">
    <h3 style="color:#646cff; font-size:14px; letter-spacing:1px;">NEXUS FORGE</h3>
    {#each files as f}
      <div on:click={() => openFile(f)} style="cursor:pointer; padding:5px; font-size:13px; color:{f.is_dir?'#e1ad01':'#aaa'}">
        {f.is_dir?'📁':'📄'} {f.name}
      </div>
    {/each}
  </div>
  <div style="flex:1; display:flex; flex-direction:column;">
    <div style="padding:10px 20px; background:#1a1a1a; border-bottom:1px solid #333; display:flex; justify-content:space-between; align-items:center;">
      <span style="font-size:12px; color:#888;">{currentFile || 'Select File'} | {status}</span>
      <button on:click={saveFile} style="background:#646cff; border:none; color:white; padding:5px 15px; border-radius:4px; cursor:pointer; font-weight:bold;">SAVE</button>
    </div>
    <textarea bind:value={editorContent} style="flex:1; background:#121212; color:#50fa7b; border:none; padding:25px; font-family:'Courier New', monospace; font-size:14px; outline:none; resize:none; line-height:1.6;"></textarea>
  </div>
</main>
