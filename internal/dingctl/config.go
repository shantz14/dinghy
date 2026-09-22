package dingctl

import (
	"log"
	"os"
	"path/filepath"

	"github.com/spf13/viper"
)

type Config struct {
	ServerAddress string `mapstructure:"API_ADDRESS"`
}

func ReadConfig() Config {
	viper.SetConfigName("config") 
	
	viper.SetConfigType("yaml")   

	home, err := os.UserHomeDir()
	if err != nil {
		log.Fatalf("Failed to find home directory: %v", err)
	}

	configPath := filepath.Join(home, ".dinghy")
	viper.AddConfigPath(configPath)      

	if err := viper.ReadInConfig(); err != nil {
		log.Fatalf("Error reading config file: %v", err)
	}

	var cfg Config
	if err := viper.Unmarshal(&cfg); err != nil {
		log.Fatalf("Unable to decode into struct: %v", err)
	}

	return cfg
}
