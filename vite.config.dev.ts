import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import wasmPackWatchPlugin from "vite-plugin-wasm-pack-watcher";

export default defineConfig({
  build: {
    watch: {
      include: ["src/**/*.ts", "src/**/*.rs"],
    },
  },
  plugins: [wasmPackWatchPlugin(), wasm(), topLevelAwait()],
});
// file:///rustc/90b35a6239c3d8bdabc530a6a0816f7ff89a0aaf/library/panic_abort/src/lib.rs
