import { defineConfig } from "orval";

export default defineConfig({
  ogonek: {
    output: {
      client: "zod",
      mode: "split",
      target: "./src/lib/types/api/gen/zod",
    },
    input: {
      target: "./openapi.json",
    },
  },
});
