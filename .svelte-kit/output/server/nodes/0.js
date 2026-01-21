

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/layout.svelte.js')).default;
export const universal = {
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.BGUTI2Ib.js","_app/immutable/chunks/Bwv-m8YG.js","_app/immutable/chunks/CPp1niMV.js","_app/immutable/chunks/Dk7Lcwng.js"];
export const stylesheets = [];
export const fonts = [];
