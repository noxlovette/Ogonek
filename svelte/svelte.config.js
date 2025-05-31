import adapter from "@sveltejs/adapter-node";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      out: "build",
      precompress: false,
    }),
    csp: {
      directives: {
        "script-src": ["self", "https://browser.sentry-cdn.com"],
        "connect-src": ["self", "https://*.ingest.sentry.io"],
      },
      reportOnly: {
        "script-src": ["self", "https://browser.sentry-cdn.com"],
        "connect-src": ["self", "https://*.ingest.sentry.io"],
        "report-uri": [process.env.SENTRY_REPORT_URL],
      },
    },
  },
};

export default config;
