import { browser } from "$app/environment";
import pino, { type Logger, type LoggerOptions } from "pino";

export enum ServerEnvironment {
  DEV = "development",
  PREV = "preview",
  STG = "staging",
  PROD = "production",
}

export type PinoLogger = Logger & {
  setLogLevel: (env: ServerEnvironment) => void;
};

// Map environment to appropriate log level
const envToLogLevel = (env: ServerEnvironment): LoggerOptions["level"] => {
  switch (env) {
    case ServerEnvironment.DEV:
      return "debug";
    case ServerEnvironment.PREV:
    case ServerEnvironment.STG:
      return "info";
    case ServerEnvironment.PROD:
      return "warn";
    default:
      return "silent";
  }
};

// Detect environment automatically
const detectedEnv =
  (process.env.NODE_ENV as ServerEnvironment) ?? ServerEnvironment.DEV;
const defaultLogLevel = envToLogLevel(detectedEnv);

// Configure base options
let pinoOptions: LoggerOptions = {
  level: defaultLogLevel,
  formatters: {
    level: (label) => ({ level: label.toUpperCase() }),
  },
};

// Browser-specific config
if (browser) {
  pinoOptions = {
    ...pinoOptions,
    browser: { asObject: false },
    transport: {
      target: "pino-pretty",
      options: {
        colorize: true,
        levelFirst: true,
        translateTime: true,
      },
    },
  };
}

// Create logger
const logger = pino(pinoOptions) as PinoLogger;

// Allow manual override if needed
logger.setLogLevel = (env: ServerEnvironment) => {
  logger.level = envToLogLevel(env) || "silent";
};

export default logger;
