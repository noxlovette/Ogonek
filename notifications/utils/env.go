package utils

import (
	"log"
	"os"

	"github.com/joho/godotenv"
)

func GetEnvVar(key string) string {
	err := godotenv.Load(".env")

	if err != nil {
		log.Fatal(err)
	}

	v, ok := os.LookupEnv(key)
	if !ok {
		log.Default().Fatalf("env variable %v not set", key)
	}
	return v
}
