import { sentrySvelteKit } from "@sentry/sveltekit";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [
    sentrySvelteKit({
      sourceMapsUploadOptions: {
        org: "danila-volkov",
        project: "svelte-fl",
      },
    }),
    sveltekit(),
  ],
  server: {
    host: true,
    port: 3000,
  },
});
