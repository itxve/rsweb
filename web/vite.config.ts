import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vite.dev/config/
export default defineConfig({
  base: "/_app/",
  plugins: [react()],
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:41218",
        changeOrigin: true,
      },
      "/ws": {
        target: "ws://localhost:41218",
        ws: true,
      },
    },
  },
});
