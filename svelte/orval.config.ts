import { defineConfig } from "orval";

export default defineConfig({
  ogonek: {
    output: {
      client: "zod",
      mode: "single",
      target: "./src/lib/types/gen/zod",
    },
    input: {
      target: "./openapi.json",
    },
  },
});
