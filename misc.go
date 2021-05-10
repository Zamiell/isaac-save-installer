package main

import (
	"fmt"
	"os"
)

func exit() {
	fmt.Println("Press enter to quit.")
	fmt.Scanln()
	os.Exit(1)
}

func fatal(msg string) {
	fmt.Println(msg)
	exit()
}

func fatalError(msg string, err error) {
	fmt.Print(msg+"\n", err)
	exit()
}

func fileExists(filePath string) bool {
	_, err := os.Stat(filePath)
	return err == nil
}
