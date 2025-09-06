import type { PlaywrightTestConfig } from "@playwright/test";

const config: PlaywrightTestConfig = {
  webServer: {
    command: "pnpm run build && pnpm run dev",
    port: 5173,
    env: {
      PUBLIC_MOCK_MODE: "true",
    },
    reuseExistingServer: true,
  },
  testDir: "playwright",
  testMatch: /(.+\.)?(test|spec)\.[jt]s/,
  use: {
    baseURL: "http://localhost:5173",
  },
};
export default config;
