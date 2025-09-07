import { defineConfig } from "orval";

export default defineConfig({
  ogonek: {
    output: {
      client: "zod",
      mode: "split",
      target: "./src/lib/zod",
    },
    input: {
      target: "./openapi.json",
    },
  },
});
