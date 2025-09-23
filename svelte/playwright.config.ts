import type { PlaywrightTestConfig } from "@playwright/test";

const config: PlaywrightTestConfig = {
  webServer: {
    command: "pnpm run build && pnpm run preview",
    port: 4173,
    env: {
      PUBLIC_MOCK_MODE: "true",
    },
    reuseExistingServer: false,
  },
  testDir: "playwright",
  testMatch: /(.+\.)?(test|spec)\.[jt]s/,
  use: {
    baseURL: "http://localhost:4173",
  },
};

export default config;
