import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [sveltekit()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // THE MASON'S SEAL: Protect against the reload loop
      ignored: [
        "**/src-tauri/**",
        "**/target/**",
        "**/lazarus_os/**",
        "**/*.rs",
        "**/*.json",
        "**/*.sh",
        "**/node_modules/**"
      ],
    },
  },
}));
