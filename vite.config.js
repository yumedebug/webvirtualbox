import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// GitHub Pages用のベースパス（リポジトリ名に合わせて変更）
// ローカル開発時は '/' を使用
const base = process.env.NODE_ENV === 'production' ? '/webvirtualbox/' : '/';

export default defineConfig({
  plugins: [react(), tailwindcss()],
  base,
  server: {
    host: "0.0.0.0",
    port: 3000,
    strictPort: true,
    hmr: {
      port: 3000,
    },
  },
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
    sourcemap: false,
  },
});
