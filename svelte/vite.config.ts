import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  server: {
    host: true,
    port: 5173,
    allowedHosts: ["next-precisely-piranha.ngrok-free.app"],
  },
  plugins: [sveltekit()],
  optimizeDeps: {
    exclude: ["fsevents"],
  },
  build: {
    rollupOptions: {
      external: ["fsevents"],
    },
  },
  ssr: {
    noExternal: process.env.NODE_ENV === "production" ? ["@carbon/charts"] : [],
  },
});
