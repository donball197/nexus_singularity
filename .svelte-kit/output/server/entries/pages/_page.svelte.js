import { w as attr_style, x as ensure_array_like, y as stringify, z as attr } from "../../chunks/index.js";
import { e as escape_html } from "../../chunks/context.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let files = [];
    let editorContent = "";
    let status = "Ready";
    let termOutput = ">> COMMANDER GEMMA-3 ONLINE\n";
    let termCmd = "";
    async function fetchFiles() {
      const res = await fetch("/api/files");
      files = await res.json();
    }
    fetchFiles();
    $$renderer2.push(`<main style="display:flex; height:100vh; background:#050505; color:#00ff00; font-family:'Courier New', monospace; overflow:hidden;"><div${attr_style(`width:${stringify("240px")}; transition:width 0.3s; border-right:1px solid #003300; background:#001100; overflow-y:auto; display:flex; flex-direction:column;`)}><div style="padding:15px;"><h3 style="color:#00ff00; font-size:14px; margin:0 0 10px 0;">NEXUS HIVE</h3> <!--[-->`);
    const each_array = ensure_array_like(files);
    for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
      let f = each_array[$$index];
      $$renderer2.push(`<div${attr_style(`cursor:pointer; padding:6px; font-size:13px; color:${stringify(f.is_dir ? "#008800" : "#00cc00")}; border-bottom:1px solid #002200;`)}>${escape_html(f.is_dir ? "📁" : "📄")} ${escape_html(f.name)}</div>`);
    }
    $$renderer2.push(`<!--]--></div></div> <div style="flex:1; display:flex; flex-direction:column; min-width:0;"><div style="padding:10px 20px; background:#001100; border-bottom:1px solid #003300; display:flex; justify-content:space-between; align-items:center;"><div style="display:flex; align-items:center; gap:15px;"><button style="background:none; border:none; color:#00ff00; cursor:pointer; font-size:18px;">☰</button> <span style="font-size:12px; color:#008800;">${escape_html("AWAITING INPUT")} | ${escape_html(status)}</span></div> <button style="background:#004400; border:1px solid #00ff00; color:#00ff00; padding:5px 15px; cursor:pointer;">SAVE</button></div> <textarea spellcheck="false" style="flex:1; background:#050505; color:#00ff00; border:none; padding:20px; font-family:'Courier New'; font-size:14px; outline:none; resize:none;">`);
    const $$body = escape_html(editorContent);
    if ($$body) {
      $$renderer2.push(`${$$body}`);
    }
    $$renderer2.push(`</textarea> <div style="height:250px; background:#000000; border-top:1px solid #003300; display:flex; flex-direction:column;"><div id="term" style="flex:1; padding:10px; overflow-y:auto; font-size:12px; white-space:pre-wrap;">${escape_html(termOutput)}</div> <div style="display:flex; border-top:1px solid #003300; padding:5px;"><span style="padding:5px 10px;">$</span> <input${attr("value", termCmd)} type="text" style="flex:1; background:transparent; border:none; color:#00ff00; font-family:'Courier New'; outline:none;" placeholder="nexus: spawn a scout named monitor_01"/></div></div></div></main>`);
  });
}
export {
  _page as default
};
