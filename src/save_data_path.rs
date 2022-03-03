use crate::{enums::IsaacVersion, utils::get_username};
use anyhow::{bail, Context, Result};
use colored::*;
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};
use winreg::enums::*;

pub fn get_steam_save_data_path(_isaac_version: IsaacVersion) -> Result<PathBuf> {
    const ISAAC_STEAM_ID: u32 = 250900;

    let steam_installation_path = get_steam_installation_path()?;
    let steam_user_id = get_steam_active_user_id()?;

    let steam_save_data_path = steam_installation_path
        .join("userdata")
        .join(steam_user_id.to_string())
        .join(ISAAC_STEAM_ID.to_string())
        .join("remote");

    Ok(steam_save_data_path)
}

fn get_steam_installation_path() -> Result<PathBuf> {
    const STEAM_REGISTRY_PATH: &str = "Software\\Valve\\Steam";
    const STEAM_PATH_KEY_VALUE: &str = "SteamPath";

    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let steam_key = hkcu.open_subkey(STEAM_REGISTRY_PATH).context(format!(
        "Failed to get the Windows registry key: {}",
        STEAM_REGISTRY_PATH
    ))?;
    let steam_path_string: String = steam_key.get_value(STEAM_PATH_KEY_VALUE).context(format!(
        "Failed to get the \"{}\" value from the Windows registry key: {}",
        STEAM_PATH_KEY_VALUE, STEAM_REGISTRY_PATH
    ))?;

    Ok(PathBuf::from(steam_path_string))
}

fn get_steam_active_user_id() -> Result<u32> {
    const ACTIVE_PROCESS_REGISTRY_PATH: &str = "Software\\Valve\\Steam\\ActiveProcess";
    const ACTIVE_USER_KEY_VALUE: &str = "ActiveUser";

    let hkcu = winreg::RegKey::predef(HKEY_CURRENT_USER);
    let active_process_key = hkcu
        .open_subkey(ACTIVE_PROCESS_REGISTRY_PATH)
        .context(format!(
            "Failed to get the Windows registry key: {}",
            ACTIVE_PROCESS_REGISTRY_PATH
        ))?;
    let active_user: u32 = active_process_key
        .get_value(ACTIVE_USER_KEY_VALUE)
        .context(format!(
            "Failed to get the \"{}\" value from the Windows registry key: {}",
            ACTIVE_USER_KEY_VALUE, ACTIVE_PROCESS_REGISTRY_PATH
        ))?;

    match active_user {
        0 => bail!("You are not currently logged into Steam. Please make sure that Steam is open and that you are logged in."),
        _ => Ok(active_user),
    }
}

pub fn get_documents_save_data_path(isaac_version: IsaacVersion) -> Result<PathBuf> {
    const LOG_TXT: &str = "log.txt";

    let username = get_username();
    let version_directory_name = get_version_directory_name(isaac_version);

    // If the user has a custom "Documents" directory, Isaac ignores this and instead puts its files
    // in the standard location
    // Test to see if the "log.txt" file exists at the "standard" location
    // e.g. "C:\Users\Alice\Documents\My Games\Binding of Isaac Repentance\log.txt"
    let standard_path = PathBuf::from(r"C:\")
        .join("Users")
        .join(username)
        .join("Documents")
        .join("My Games")
        .join(&version_directory_name);
    let standard_log_path = standard_path.join(LOG_TXT);
    if standard_log_path.exists() {
        return Ok(standard_path);
    }

    // The standard documents location does not seem to exist, so the user might have a "Documents"
    // directory that is in a custom location
    // The "dirs_next" library queries the Windows API to determine this
    let documents_path = dirs_next::document_dir()
        .context("Unable to find the path to your \"Documents\" directory.")?;
    let custom_path = documents_path
        .join("My Games")
        .join(&version_directory_name);
    let custom_log_path = custom_path.join(LOG_TXT);
    if custom_log_path.exists() {
        return Ok(custom_path);
    }

    let path_string = custom_path.to_str().context(format!(
        "Failed to convert the path to a string: {}",
        custom_path.display()
    ))?;
    bail!(
        "Failed to find your documents save data directory at:\n{}\n\nDo you have this version of the game installed?",
        path_string.green(),
    )
}

fn get_version_directory_name(isaac_version: IsaacVersion) -> String {
    let directory_name = match isaac_version {
        IsaacVersion::Rebirth => "Binding of Isaac Rebirth",
        IsaacVersion::Afterbirth => "Binding of Isaac Afterbirth",
        IsaacVersion::AfterbirthPlus | IsaacVersion::AfterbirthPlusBP5 => {
            "Binding of Isaac Afterbirth+"
        }
        IsaacVersion::Repentance => "Binding of Isaac Repentance",
    };

    String::from(directory_name)
}

pub fn get_steam_cloud_enabled(save_data_path: &Path) -> Result<bool> {
    const OPTIONS_INI: &str = "options.ini";
    const OPTIONS_SECTION_NAME: &str = "Options";
    const STEAM_CLOUD_NAME: &str = "SteamCloud";

    let options_ini_path = save_data_path.join(OPTIONS_INI);
    if !&options_ini_path.exists() {
        bail!(
            "Failed to find your \"{}\" file at \": {}",
            OPTIONS_INI,
            options_ini_path.display(),
        );
    }

    let options_ini_string = read_to_string(&options_ini_path).context(format!(
        "Failed to read the file: {}",
        options_ini_path.display()
    ))?;

    let options_ini = ini::Ini::load_from_str(&options_ini_string).context(format!(
        "Failed to parse the file: {}",
        options_ini_path.display()
    ))?;

    let options_section = options_ini
        .section(Some(OPTIONS_SECTION_NAME))
        .context(format!(
            "The \"{}\" file does not have a section called: {}",
            OPTIONS_INI, OPTIONS_SECTION_NAME,
        ))?;

    let steam_cloud_string = options_section.get(STEAM_CLOUD_NAME).context(format!(
        "The \"{}\" file does not have a key called: {}",
        OPTIONS_INI, STEAM_CLOUD_NAME,
    ))?;

    match steam_cloud_string {
        "0" => Ok(false),
        "1" => Ok(true),
        _ => bail!(
            "The value for the \"{}\" key is invalid: {}",
            STEAM_CLOUD_NAME,
            steam_cloud_string,
        ),
    }
}
