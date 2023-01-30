import { defineConfig, loadEnv } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import path from 'path';

// https://vitejs.dev/config/
export default ({ mode }) => {
  process.env = {...process.env, ...loadEnv(mode, process.cwd())}

  return defineConfig({
    plugins: [svelte({ configFile: "../svelte.config.js" })],
    root: "src/",
    publicDir: "../public",
    resolve: {
      alias: {
        $lib: path.resolve("./src/lib"),
      }
    },
    build: {
      emptyOutDir: true,
      outDir: "../dist/default",
      rollupOptions: {
        preserveEntrySignatures: "exports-only",
        input: {
          options: "src/options/index.html",
          popup: "src/popup/index.html",
          background: "src/background/index.html",
        },
      },
    },
  });
};
