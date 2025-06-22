package utils

import (
	"fmt"

	"go.uber.org/zap"
)

func LogErr(logger *zap.Logger, msg string, err error) error {
	logger.Error(msg, zap.Error(err))
	return fmt.Errorf("%s: %w", msg, err)
}
