// redis.ts (or add types to your .js file)
import { env } from "$env/dynamic/private";
import type { Redis } from "ioredis";

interface MockRedis {
  get: () => Promise<null>;
  set: () => Promise<string>;
  setex: () => Promise<string>;
  del: () => Promise<number>;
  exists: () => Promise<number>;
  on: () => void;
  connect: () => Promise<void>;
  disconnect: () => Promise<void>;
}

let redis: Redis | MockRedis;

if (env.MOCK_MODE) {
  // Mock Redis for tests
  redis = {
    get: async () => null,
    set: async () => "OK",
    setex: async () => "OK",
    del: async () => 1,
    exists: async () => 0,
    on: () => {},
    connect: async () => {},
    disconnect: async () => {},
  };
} else {
  // Real Redis for dev/prod
  const Redis = (await import("ioredis")).default;
  redis = new Redis({
    host: process.env.REDIS_HOST || "redis",
    port: parseInt(process.env.REDIS_PORT || "6379", 10),
    lazyConnect: true,
  });
}

export default redis;
