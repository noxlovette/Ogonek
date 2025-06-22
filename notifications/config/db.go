package config

import (
	"os"
	"time"
)

type DbConfig struct {
	ConnectionString string
	MaxConns         int32
	MinConns         int32
	MaxConnLifetime  time.Duration
	MaxConnIdleTime  time.Duration
}

func DefaultDbConfig() DbConfig {
	return DbConfig{
		ConnectionString: os.Getenv("DATABASE_URL"),
		MaxConns:         10,
		MinConns:         2,
		MaxConnLifetime:  time.Hour,
		MaxConnIdleTime:  time.Minute * 30,
	}
}
