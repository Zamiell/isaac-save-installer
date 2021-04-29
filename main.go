package main

import (
	"encoding/base64"
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"path/filepath"
	"strings"

	ps "github.com/mitchellh/go-ps"
)

const (
	DefaultFileMode = 0644
)

func main() {
	printBanner()
	getUserToAgreeToDeletingSaveFiles()
	checkIsIsaacOpen()
	repentanceSaveDataPath := getRepentanceSaveDataPath()
	disableSteamCloud(repentanceSaveDataPath)
	deleteExistingSaveFiles(repentanceSaveDataPath)
	installSaveFile(repentanceSaveDataPath)

	fmt.Println("A fully unlocked save file has been installed to save slot 1.")
}

func printBanner() {
	fmt.Println("+------------------------------------+")
	fmt.Println("|  The Binding of Isaac: Repentance  |")
	fmt.Println("| Fully Unlocked Save File Installer |")
	fmt.Println("+------------------------------------+")
	fmt.Println()
}

func getUserToAgreeToDeletingSaveFiles() {
	// Skip getting user input if they specified the "--yes" flag
	if len(os.Args) > 1 && os.Args[1] == "--yes" {
		return
	}

	fmt.Println("Hello. I will now install a fully unlocked save file.")
	fmt.Println("Type \"yes\" and press enter if you agree that I can delete any existing save files, if any.")
	var userInput string
	fmt.Scanln(&userInput)
	fmt.Println()
	if userInput != "yes" {
		fatal("Ok then. Manually back up your save files and then summon me again.")
	}
}

func checkIsIsaacOpen() {
	var processes []ps.Process
	if v, err := ps.Processes(); err != nil {
		fatalError("Failed to get the list of processes: %v", err)
	} else {
		processes = v
	}

	for _, process := range processes {
		if process.Executable() == "isaac-ng.exe" {
			fatal("Error: You are currently running The Binding of Isaac: Repentance. Close the game before you run this installer.")
		}
	}
}

func getRepentanceSaveDataPath() string {
	// From: https://superuser.com/questions/1132288/windows-command-prompt-get-relocated-users-documents-folder
	cmd := exec.Command("powershell", "[Environment]::GetFolderPath('MyDocuments')")
	var documentsDir string
	if output, err := cmd.Output(); err != nil {
		fatalError("Failed to get the documents directory: %v", err)
	} else {
		documentsDir = string(output)
		documentsDir = strings.TrimSpace(documentsDir)
	}

	// Must use "filepath" instead of "path" to avoid Windows bugs
	return filepath.Join(documentsDir, "My Games", "Binding of Isaac Repentance")
}

func disableSteamCloud(repentanceSaveDataPath string) {
	optionsINIPath := filepath.Join(repentanceSaveDataPath, "options.ini")

	// Check to see if the "options.ini" file exists
	if !fileExists(optionsINIPath) {
		fatal("Failed to find your \"options.ini\" file at \"" + optionsINIPath + "\".")
	}

	// Read the "options.ini" file
	var optionsINI string
	if v, err := ioutil.ReadFile(optionsINIPath); err != nil {
		fatalError("Failed to read your \"options.ini\" file:", err)
	} else {
		optionsINI = string(v)
	}

	// As a sanity check, confirm that there is a SteamCloud value set
	if !strings.Contains(optionsINI, "SteamCloud=0") && !strings.Contains(optionsINI, "SteamCloud=1") {
		fatal("Failed to parse your \"options.ini\" file.")
	}

	// If SteamCloud is enabled, turn it off
	if strings.Contains(optionsINI, "SteamCloud=1") {
		optionsINI = strings.ReplaceAll(optionsINI, "SteamCloud=1", "SteamCloud=0")
		if err := ioutil.WriteFile(optionsINIPath, []byte(optionsINI), DefaultFileMode); err != nil {
			fatalError("Failed to write to the \"options.ini\" file:", err)
		}
	}
}

func deleteExistingSaveFiles(repentanceSaveDataPath string) {
	for i := 1; i <= 3; i++ {
		fileName := fmt.Sprintf("persistentgamedata%d.dat", i)
		filePath := filepath.Join(repentanceSaveDataPath, fileName)
		if fileExists(filePath) {
			if err := os.Remove(filePath); err != nil {
				fatalError("Failed to remove the \""+filePath+"\" file:", err)
			}
		}
	}
}

func installSaveFile(repentanceSaveDataPath string) {
	safeFilePath := filepath.Join(repentanceSaveDataPath, "persistentgamedata1.dat")
	saveFile := unpackSaveFile()
	if err := ioutil.WriteFile(safeFilePath, saveFile, DefaultFileMode); err != nil {
		fatalError("Failed to write to the \"options.ini\" file:", err)
	}
}

func unpackSaveFile() []byte {
	safeFileBase64Trimmed := strings.TrimSpace(saveFileBase64)
	if decoded, err := base64.StdEncoding.DecodeString(safeFileBase64Trimmed); err != nil {
		fatalError("Failed to decode the save file: %v", err)
		return nil
	} else {
		return decoded
	}
}
