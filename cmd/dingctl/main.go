package main

import (
	"os"

	"github.com/shantz14/dinghy/internal/dingctl"
)

func main() {
	args := os.Args
	dingctl.Execute(args)
}

