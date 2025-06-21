import { sveltekit } from "@sveltejs/kit/vite";

export default {
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
};
