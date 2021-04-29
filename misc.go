package main

import (
	"fmt"
	"log"
	"os"
)

func exit() {
	fmt.Println("Press enter to quit.")
	fmt.Scanln()
	os.Exit(1)
}

func fatal(msg string) {
	log.Println(msg)
	exit()
}

func fatalError(msg string, err error) {
	log.Printf(msg+"\n", err)
	exit()

}

func fileExists(filePath string) bool {
	_, err := os.Stat(filePath)
	return err == nil
}
