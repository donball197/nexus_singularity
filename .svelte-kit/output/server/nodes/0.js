

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/layout.svelte.js')).default;
export const universal = {
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.BAFxEw-9.js","_app/immutable/chunks/YgD__NZA.js","_app/immutable/chunks/Cs9VTpyV.js","_app/immutable/chunks/DbybB0K8.js"];
export const stylesheets = [];
export const fonts = [];
