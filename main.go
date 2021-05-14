package main

import (
	"encoding/base64"
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
	"os/user"
	"path/filepath"
	"strings"

	ps "github.com/mitchellh/go-ps"
)

const (
	DefaultFileMode = 0644
)

func main() {
	printBanner()
	checkIsIsaacOpen()
	getUserToAgreeToDeletingSaveFiles()
	isaacVersion := getUserIsaacVersion()
	saveDataPath := getSaveDataPath(isaacVersion)
	disableSteamCloud(saveDataPath)
	deleteExistingSaveFiles(saveDataPath)
	installSaveFiles(isaacVersion, saveDataPath)
	confirmExit()
}

func printBanner() {
	fmt.Println("+------------------------------------+")
	fmt.Println("|   The Binding of Isaac: Rebirth    |")
	fmt.Println("|             (and DLCs)             |")
	fmt.Println("| Fully Unlocked Save File Installer |")
	fmt.Println("+------------------------------------+")
	fmt.Println()
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
			fatal("Error: You are currently running The Binding of Isaac: Rebirth. Close the game before you run this installer.")
		}
	}
}

func getUserToAgreeToDeletingSaveFiles() {
	// Skip getting user input if they specified the "--yes" flag
	if len(os.Args) > 1 && os.Args[1] == "--yes" {
		return
	}

	fmt.Println("Hello. I will install a fully unlocked save file for you.")
	fmt.Println("Type \"yes\" and press enter if you agree that I can delete existing save files, if any.")

	var userInput string
	fmt.Scanln(&userInput)
	fmt.Println()
	if userInput != "yes" {
		fatal("Ok then. Manually back up your save files and then summon me again.")
	}
}

func getUserIsaacVersion() IsaacVersion {
	fmt.Println("Which game do you want to install a save file for?")
	fmt.Println("1) The Binding of Isaac: Rebirth")
	fmt.Println("2) The Binding of Isaac: Afterbirth")
	fmt.Println("3) The Binding of Isaac: Afterbirth+ (Vanilla through Booster Pack 4)")
	fmt.Println("4) The Binding of Isaac: Afterbirth+ (Booster Pack 5)")
	fmt.Println("5) The Binding of Isaac: Repentance")
	fmt.Println("[Type the number and press enter.]")

	var userInput string
	fmt.Scanln(&userInput)
	fmt.Println()

	var isaacVersion IsaacVersion
	switch userInput {
	case "1":
		isaacVersion = Rebirth
	case "2":
		isaacVersion = Afterbirth
	case "3":
		isaacVersion = AfterbirthPlus
	case "4":
		isaacVersion = AfterbirthPlusBP5
	case "5":
		isaacVersion = Repentance
	default:
		fatal("That is not a valid option. Exiting.")
	}

	return isaacVersion
}

func getSaveDataPath(isaacVersion IsaacVersion) string {
	username := getUsername()
	fmt.Println("A", username, "B")

	// If the user has a custom "Documents" directory, Isaac ignores this and instead puts its files in the standard location
	// Test to see if the log.txt exists at the "standard" location
	// e.g. "C:\Users\Alice\Documents\My Games\Binding of Isaac Repentance\log.txt"
	// (we must use "filepath" instead of "path" to avoid Windows bugs)
	versionFolder := getVersionFolder(isaacVersion)
	standardSaveDataPath := filepath.Join("C:\\", "Users", username, "Documents", "My Games", versionFolder)
	standardLogPath := filepath.Join(standardSaveDataPath, "log.txt")
	if fileExists(standardLogPath) {
		return standardSaveDataPath
	}

	// From: https://superuser.com/questions/1132288/windows-command-prompt-get-relocated-users-documents-folder
	cmd := exec.Command("powershell", "[Environment]::GetFolderPath('MyDocuments')")
	var documentsDir string
	if output, err := cmd.Output(); err != nil {
		fatalError("Failed to get the documents directory: %v", err)
	} else {
		documentsDir = string(output)
		documentsDir = strings.TrimSpace(documentsDir)
	}

	saveDataPath := filepath.Join(documentsDir, "My Games", versionFolder)
	if fileExists(saveDataPath) {
		return saveDataPath
	}

	fatal("Failed to find your save data directory at \"" + saveDataPath + "\".")
	return ""
}

func getUsername() string {
	var rawUsername string
	if v, err := user.Current(); err != nil {
		fatalError("Failed to get the current user: %v", err)
	} else {
		rawUsername = v.Username // e.g. alice-computer\Alice
	}

	usernameParts := strings.Split(rawUsername, "\\")
	if len(usernameParts) == 0 {
		return rawUsername
	}

	return usernameParts[len(usernameParts)-1]
}

func disableSteamCloud(saveDataPath string) {
	optionsINIPath := filepath.Join(saveDataPath, "options.ini")

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

func installSaveFiles(isaacVersion IsaacVersion, saveDataPath string) {
	for slot := 1; slot <= 3; slot++ {
		installSaveFile(isaacVersion, saveDataPath, slot)
	}
}

func installSaveFile(isaacVersion IsaacVersion, saveDataPath string, slot int) {
	fileName := fmt.Sprintf("persistentgamedata%d.dat", slot)
	safeFilePath := filepath.Join(saveDataPath, fileName)
	saveFile := unpackSaveFile(isaacVersion)
	if err := ioutil.WriteFile(safeFilePath, saveFile, DefaultFileMode); err != nil {
		fatalError("Failed to write to the \""+safeFilePath+"\" file:", err)
	}
}

func unpackSaveFile(isaacVersion IsaacVersion) []byte {
	saveFileBase64 := getVersionSaveFileBase64(isaacVersion)
	safeFileBase64Trimmed := strings.TrimSpace(saveFileBase64)
	if decoded, err := base64.StdEncoding.DecodeString(safeFileBase64Trimmed); err != nil {
		fatalError("Failed to decode the save file: %v", err)
		return nil
	} else {
		return decoded
	}
}

func confirmExit() {
	fmt.Println("A fully unlocked save file has been installed to all 3 save slots.")
	fmt.Println("Press enter to exit.")
	var userInput string
	fmt.Scanln(&userInput)
}
