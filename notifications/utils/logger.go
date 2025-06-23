package utils

import (
	"fmt"
	"os"

	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

func LogErr(logger *zap.Logger, msg string, err error) error {
	logger.Error(msg, zap.Error(err))
	return fmt.Errorf("%s: %w", msg, err)
}

func IsDevEnv() bool {
	env := os.Getenv("APP_ENV")
	return env == "" || env == "development"
}

func NewLogger() (*zap.Logger, error) {
	if IsDevEnv() {
		cfg := zap.NewDevelopmentEncoderConfig()
		cfg.EncodeLevel = zapcore.CapitalColorLevelEncoder
		cfg.EncodeTime = zapcore.ISO8601TimeEncoder

		consoleEncoder := zapcore.NewConsoleEncoder(cfg)
		core := zapcore.NewCore(consoleEncoder, zapcore.Lock(os.Stdout), zapcore.DebugLevel)

		return zap.New(core, zap.AddCaller()), nil
	}

	// Default: production JSON logger
	return zap.NewProduction()
}
