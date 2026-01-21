export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png","index.html","neural_goal.json","svelte.svg","tauri.svg","vite.svg"]),
	mimeTypes: {".png":"image/png",".html":"text/html",".json":"application/json",".svg":"image/svg+xml"},
	_: {
		client: {start:"_app/immutable/entry/start.DXsCjTpU.js",app:"_app/immutable/entry/app.l9PIx7xI.js",imports:["_app/immutable/entry/start.DXsCjTpU.js","_app/immutable/chunks/YSHuM2iD.js","_app/immutable/chunks/CPp1niMV.js","_app/immutable/chunks/BDyckQHn.js","_app/immutable/entry/app.l9PIx7xI.js","_app/immutable/chunks/CPp1niMV.js","_app/immutable/chunks/C3KH7Zr0.js","_app/immutable/chunks/Bwv-m8YG.js","_app/immutable/chunks/BDyckQHn.js","_app/immutable/chunks/Dk7Lcwng.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
