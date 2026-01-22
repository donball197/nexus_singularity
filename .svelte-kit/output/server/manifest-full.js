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
		client: {start:"_app/immutable/entry/start.Dbz5ZhNh.js",app:"_app/immutable/entry/app.DfTMsgNt.js",imports:["_app/immutable/entry/start.Dbz5ZhNh.js","_app/immutable/chunks/DXVC1-Fd.js","_app/immutable/chunks/Cs9VTpyV.js","_app/immutable/chunks/BtY-nPd6.js","_app/immutable/entry/app.DfTMsgNt.js","_app/immutable/chunks/Cs9VTpyV.js","_app/immutable/chunks/6QcDtyMw.js","_app/immutable/chunks/YgD__NZA.js","_app/immutable/chunks/BtY-nPd6.js","_app/immutable/chunks/DbybB0K8.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
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
			},
			{
				id: "/api/ask",
				pattern: /^\/api\/ask\/?$/,
				params: [],
				page: null,
				endpoint: __memo(() => import('./entries/endpoints/api/ask/_server.js'))
			},
			{
				id: "/api/files",
				pattern: /^\/api\/files\/?$/,
				params: [],
				page: null,
				endpoint: __memo(() => import('./entries/endpoints/api/files/_server.js'))
			},
			{
				id: "/api/read",
				pattern: /^\/api\/read\/?$/,
				params: [],
				page: null,
				endpoint: __memo(() => import('./entries/endpoints/api/read/_server.js'))
			},
			{
				id: "/api/save",
				pattern: /^\/api\/save\/?$/,
				params: [],
				page: null,
				endpoint: __memo(() => import('./entries/endpoints/api/save/_server.js'))
			},
			{
				id: "/api/term/exec",
				pattern: /^\/api\/term\/exec\/?$/,
				params: [],
				page: null,
				endpoint: __memo(() => import('./entries/endpoints/api/term/exec/_server.js'))
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
