package dingctl

import "fmt"

func Execute(args []string) {
	if len(args) == 1 {
		printHelp()
		return
	}
	cmd := args[1]
	switch (cmd) {
	case "apply":
		apply(args[2:])
	}
}

func printHelp() {
	fmt.Println("A tool for interacting with the dinghy API through the command line.")
	fmt.Println("")
	fmt.Println("Usage:")
}

