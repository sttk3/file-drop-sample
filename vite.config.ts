import { defineConfig } from "vite" ;
import preact from "@preact/preset-vite" ;
import { resolve } from 'path' ;
const __dirname = import.meta.dirname ;

const root = resolve(__dirname, "src") ;
const outDir = resolve(__dirname, "dist") ;

// \@ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST ;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  root: root,

  plugins: [
    preact(), 
  ], 

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false, 

  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420, 
    // port: 5000, 
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
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"], 
    }, 
  }, 

  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",

    chunkSizeWarningLimit: 2000,
    outDir: outDir,
    emptyOutDir: true,
    rollupOptions: {
      input: {
        window_main: resolve(root, "index.html"),
      },
    }, 
  }, 
})) ;
