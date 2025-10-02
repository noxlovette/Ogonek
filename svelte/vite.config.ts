import { paraglideVitePlugin } from "@inlang/paraglide-js";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import { configDefaults } from "vitest/config";

export default defineConfig({
  server: {
    host: true,
    port: 5173,
    allowedHosts: [
      "next-precisely-piranha.ngrok-free.app",
      "staging.ogonek.app",
    ],
  },
  plugins: [
    sveltekit(),
    paraglideVitePlugin({
      project: "./project.inlang",
      outdir: "./src/lib/paraglide",
      strategy: ["cookie", "baseLocale"],
    }),
  ],
  optimizeDeps: {
    exclude: ["fsevents"],
    force: true,
    include: ["@lucide/svelte"]
  },
  build: {
    rollupOptions: { external: ["fsevents", "src/lib/routes/api/mock"] },
  },
  ssr: {
    noExternal: process.env.NODE_ENV === "production" ? ["@carbon/charts"] : [],
  },
  test: {
    include: ["src/**/*.{test,spec}.{js,ts}"],
    exclude: [...configDefaults.exclude, "playwright/**/*"],
    environment: "node",
    globals: true,
  },
});
