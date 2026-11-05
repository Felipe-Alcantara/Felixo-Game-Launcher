import { defineConfig } from "vitest/config";

// A interface vive em `ui/`; o Tauri carrega o resultado de `dist/`.
export default defineConfig({
  root: "ui",
  clearScreen: false,
  server: { port: 1420, strictPort: true },
  build: { outDir: "../dist", emptyOutDir: true, target: "es2022" },
  test: { include: ["src/**/*.test.ts"], environment: "node" },
});
