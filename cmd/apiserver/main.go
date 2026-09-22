package main

import (
	"os"

	"github.com/shantz14/dinghy/internal/apiserver"
)

func main() {
	args := os.Args

	port := args[1]

	s := apiserver.NewServer(port)
	s.Run()
}
