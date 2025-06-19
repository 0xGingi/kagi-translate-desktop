import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [],

  // Include the public directory
  publicDir: "public",

  // Ensure public files are copied
  build: {
    outDir: "dist",
    assetsDir: "assets",
  },

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
      ignored: ["**/src-tauri/**"],
    },
  },
}));
