import type { PlaywrightTestConfig } from "@playwright/test";

const config: PlaywrightTestConfig = {
  webServer: {
    command: "pnpm run build && pnpm run preview",
    port: 4173,
    env: {
      NODE_ENV: "test",
      PUBLIC_MOCK_MODE: "true",
    },
  },
  testDir: "tests",
  testMatch: /(.+\.)?(test|spec)\.[jt]s/,
  use: {
    baseURL: "http://localhost:4173",
  },
};
export default config;
