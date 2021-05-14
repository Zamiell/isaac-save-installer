package main

import (
	"fmt"
)

type IsaacVersion int

const (
	Rebirth IsaacVersion = iota
	Afterbirth
	AfterbirthPlus
	AfterbirthPlusBP5
	Repentance
)

func getVersionFolder(isaacVersion IsaacVersion) string {
	switch isaacVersion {
	case Rebirth:
		return "Binding of Isaac Rebirth"
	case Afterbirth:
		return "Binding of Isaac Afterbirth"
	case AfterbirthPlus, AfterbirthPlusBP5:
		return "Binding of Isaac Afterbirth+"
	case Repentance:
		return "Binding of Isaac Repentance"
	}

	msg := fmt.Sprintf("Unknown Isaac version of: %d", isaacVersion)
	fatal(msg)
	return ""
}

func getVersionSaveFileBase64(isaacVersion IsaacVersion) string {
	switch isaacVersion {
	case Rebirth:
		return saveFileRebirthBase64
	case Afterbirth:
		return saveFileAfterbirthBase64
	case AfterbirthPlus:
		return saveFileAfterbirthPlusBase64
	case AfterbirthPlusBP5:
		return saveFileAfterbirthPlusBP5Base64
	case Repentance:
		return saveFileRepentanceBase64
	}

	msg := fmt.Sprintf("Unknown Isaac version of: %d", isaacVersion)
	fatal(msg)
	return ""
}
