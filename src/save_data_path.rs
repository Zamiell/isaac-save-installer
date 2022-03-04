use crate::{
    constants::{OPTIONS_INI, OPTIONS_SECTION_NAME, STEAM_CLOUD_NAME},
    enums::IsaacVersion,
};
use anyhow::{bail, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use colored::Colorize;
use std::fs::read_to_string;
use winreg::enums::*;

pub fn get_steam_save_data_path() -> Result<Utf8PathBuf> {
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

fn get_steam_installation_path() -> Result<Utf8PathBuf> {
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
    let steam_path = Utf8PathBuf::from(steam_path_string);

    Ok(steam_path)
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

pub fn get_documents_save_data_path(isaac_version: IsaacVersion) -> Result<Utf8PathBuf> {
    const LOG_TXT: &str = "log.txt";

    let username = get_username();
    let version_directory_name = get_version_directory_name(isaac_version);

    // If the user has a custom "Documents" directory, Isaac ignores this and instead puts its files
    // in the standard location
    // Test to see if the "log.txt" file exists at the "standard" location
    // e.g. "C:\Users\Alice\Documents\My Games\Binding of Isaac Repentance\log.txt"
    let standard_path = Utf8PathBuf::from(r"C:\")
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
    let documents_path_utf8_result = Utf8PathBuf::from_path_buf(documents_path);
    let documents_path_utf8 = match documents_path_utf8_result {
        Ok(path_buf) => path_buf,
        Err(path_buf) => bail!(format!(
            "Failed to convert the following path to UTF8:\n{:?}",
            path_buf,
        )),
    };
    let custom_path = documents_path_utf8
        .join("My Games")
        .join(&version_directory_name);
    let custom_log_path = custom_path.join(LOG_TXT);
    if custom_log_path.exists() {
        return Ok(custom_path);
    }

    bail!(
        "Failed to find your documents save data directory at:\n{}\n\nDo you have the selected version of the game installed?",
        custom_log_path.to_string().green(),
    )
}

fn get_username() -> String {
    whoami::username()
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

pub fn get_steam_cloud_enabled(documents_save_data_path: &Utf8Path) -> Result<bool> {
    let options_ini = get_options_ini(documents_save_data_path)?;

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

fn get_options_ini(documents_save_data_path: &Utf8Path) -> Result<ini::Ini> {
    let options_ini_path = get_options_ini_path(documents_save_data_path)?;

    let options_ini_string = read_to_string(&options_ini_path).context(format!(
        "Failed to read the file:\n{}",
        options_ini_path.to_string().green(),
    ))?;

    let options_ini = ini::Ini::load_from_str(&options_ini_string).context(format!(
        "Failed to parse the file:\n{}",
        options_ini_path.to_string().green(),
    ))?;

    Ok(options_ini)
}

fn get_options_ini_path(documents_save_data_path: &Utf8Path) -> Result<Utf8PathBuf> {
    let options_ini_path = documents_save_data_path.join(OPTIONS_INI);

    if !options_ini_path.exists() {
        bail!(
            "Failed to find your \"{}\" file at \":\n{}",
            OPTIONS_INI,
            options_ini_path.to_string().green(),
        );
    }

    Ok(options_ini_path)
}

pub fn toggle_steam_cloud_enabled(
    documents_save_data_path: &Utf8Path,
    previously_enabled: bool,
) -> Result<()> {
    let mut options_ini = get_options_ini(documents_save_data_path)?;
    let mut options_section = options_ini.with_section(Some(OPTIONS_SECTION_NAME));
    let toggled_setting = match previously_enabled {
        true => "0",
        false => "1",
    };
    options_section.set(STEAM_CLOUD_NAME, toggled_setting);

    let options_ini_path = get_options_ini_path(documents_save_data_path)?;
    options_ini.write_to_file(&options_ini_path)?;

    println!(
        "Successfully set the \"{}\" value to \"{}\" in the following file:\n{}",
        STEAM_CLOUD_NAME,
        toggled_setting,
        options_ini_path.to_string().green(),
    );

    Ok(())
}
