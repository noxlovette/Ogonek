package config

import (
	"context"
	"notifications/utils"

	"go.uber.org/zap"
)

type App struct {

	Logger       *zap.Logger
	Env          *Env
}

func InitAppState(ctx context.Context) (*App, error) {

	env := &Env{
		DatabaseURL: utils.GetEnvVar("DATABASE_URL"),
		TelegramAPI: utils.GetEnvVar("TELEGRAM_API"),
	}

	logger, err := zap.NewDevelopment()
	if err != nil {
		return nil, err
	}

	

	return &App{
		Logger:       logger,
		Env:          env, 
}, nil
}