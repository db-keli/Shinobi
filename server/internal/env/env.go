package env

import (
	"os"
	"strconv"
	"time"
)

func GetString(key, fallback string) string {
	value, ok := os.LookupEnv(key)
	if !ok {
		return fallback
	}
	return value
}

func GetInt(key string, fallback int) int {
	value, ok := os.LookupEnv(key)
	if !ok {
		return fallback
	}

	valueAsInt, err := strconv.Atoi(value)
	if err != nil {
		return fallback
	}

	return valueAsInt
}

func GetDuration(key, fallback string) time.Duration {
	duration, err := time.ParseDuration(fallback)
	if err != nil {
		return 0
	}
	return duration
}
