
// this file is generated — do not edit it


declare module "svelte/elements" {
	export interface HTMLAttributes<T> {
		'data-sveltekit-keepfocus'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-noscroll'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-preload-code'?:
			| true
			| ''
			| 'eager'
			| 'viewport'
			| 'hover'
			| 'tap'
			| 'off'
			| undefined
			| null;
		'data-sveltekit-preload-data'?: true | '' | 'hover' | 'tap' | 'off' | undefined | null;
		'data-sveltekit-reload'?: true | '' | 'off' | undefined | null;
		'data-sveltekit-replacestate'?: true | '' | 'off' | undefined | null;
	}
}

export {};


declare module "$app/types" {
	export interface AppTypes {
		RouteId(): "/" | "/api" | "/api/ask" | "/api/files" | "/api/read" | "/api/save" | "/api/term" | "/api/term/exec";
		RouteParams(): {
			
		};
		LayoutParams(): {
			"/": Record<string, never>;
			"/api": Record<string, never>;
			"/api/ask": Record<string, never>;
			"/api/files": Record<string, never>;
			"/api/read": Record<string, never>;
			"/api/save": Record<string, never>;
			"/api/term": Record<string, never>;
			"/api/term/exec": Record<string, never>
		};
		Pathname(): "/" | "/api" | "/api/" | "/api/ask" | "/api/ask/" | "/api/files" | "/api/files/" | "/api/read" | "/api/read/" | "/api/save" | "/api/save/" | "/api/term" | "/api/term/" | "/api/term/exec" | "/api/term/exec/";
		ResolvedPathname(): `${"" | `/${string}`}${ReturnType<AppTypes['Pathname']>}`;
		Asset(): "/favicon.png" | "/index.html" | "/neural_goal.json" | "/svelte.svg" | "/tauri.svg" | "/vite.svg" | string & {};
	}
}