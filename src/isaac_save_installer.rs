use crate::{
    backup::backup,
    change_steam_cloud::change_steam_cloud,
    enums::{Activity, IsaacVersion},
    get_input::{prompt_for_activity, prompt_for_isaac_version, prompt_for_save_file_slot},
    install::install,
    save_data_path::{
        get_documents_save_data_path, get_steam_cloud_enabled, get_steam_save_data_path,
    },
};
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use sysinfo::{System, SystemExt};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn isaac_save_installer() -> Result<()> {
    print_banner();
    check_if_isaac_open()?;

    let isaac_version = prompt_for_isaac_version()?;
    let steam_save_data_path = get_steam_save_data_path(isaac_version)?;
    let documents_save_data_path = get_documents_save_data_path(isaac_version)?;
    let steam_cloud_enabled = get_steam_cloud_enabled(&documents_save_data_path)?;
    let save_data_path = match steam_cloud_enabled {
        true => &steam_save_data_path,
        false => &documents_save_data_path,
    };

    let existing_save_files =
        get_existing_save_files(isaac_version, save_data_path, steam_cloud_enabled);
    print_save_files(&existing_save_files);

    let activity = prompt_for_activity()?;
    if activity == Activity::ChangeSteamCloud {
        return change_steam_cloud(&documents_save_data_path, steam_cloud_enabled);
    }

    let save_file_slot = prompt_for_save_file_slot(activity)?;
    let save_file_index = save_file_slot - 1;
    let save_file = existing_save_files.get(save_file_index).context(format!(
        "Failed to get the save file at index: {}",
        save_file_index
    ))?;

    match activity {
        Activity::Backup => backup(save_file, save_file_slot),
        Activity::Install => install(save_file, isaac_version),
        _ => Ok(()),
    }
}

fn print_banner() {
    println!("+------------------------------------+");
    println!("|   The Binding of Isaac: Rebirth    |");
    println!("|             (and DLCs)             |");
    println!("| Fully Unlocked Save File Installer |");
    println!("|               v{}               |", VERSION);
    println!("+------------------------------------+");
    println!();
    println!("If you have any problems with this installer, you can get help in the");
    println!("Isaac Speedrunning & Racing Discord server:");
    println!("https://discord.com/invite/0Sokdog3miAGKovs");
    println!();
}

fn check_if_isaac_open() -> Result<()> {
    let system = System::new_all();
    let isaac_processes = system.processes_by_exact_name("isaac-ng.exe");

    match isaac_processes.count() {
        0 => Ok(()),
        _ => bail!("You are currently running The Binding of Isaac: Rebirth.\nClose the game before you run this installer."),
    }
}

fn get_existing_save_files(
    isaac_version: IsaacVersion,
    save_data_path: &Path,
    steam_cloud_enabled: bool,
) -> Vec<(PathBuf, bool)> {
    const NUM_SAVE_FILES: u32 = 3;

    let prefix = get_file_name_prefix(isaac_version, steam_cloud_enabled);

    let mut save_file_paths: Vec<(PathBuf, bool)> = Vec::new();
    for i in 0..NUM_SAVE_FILES {
        let file_name = format!("{}persistentgamedata{}.dat", prefix, i + 1);
        let save_file_path = save_data_path.join(file_name);
        let exists = save_file_path.exists();
        let tuple = (save_file_path, exists);
        save_file_paths.push(tuple);
    }

    save_file_paths
}

fn get_file_name_prefix(isaac_version: IsaacVersion, steam_cloud_enabled: bool) -> String {
    if !steam_cloud_enabled {
        return String::from("");
    }

    let prefix = match isaac_version {
        IsaacVersion::Rebirth => "",
        IsaacVersion::Afterbirth => "ab_",
        IsaacVersion::AfterbirthPlus | IsaacVersion::AfterbirthPlusBP5 => "abp_",
        IsaacVersion::Repentance => "rep_",
    };

    String::from(prefix)
}

fn print_save_files(existing_save_files: &[(PathBuf, bool)]) {
    println!("Your current save files are as follows:");
    for (i, (save_file_path, exists)) in existing_save_files.iter().enumerate() {
        let value = match exists {
            true => save_file_path.to_str().unwrap_or("[unknown]"),
            false => "[empty]",
        };
        println!("{}) {}", i + 1, value);
    }
    println!();
}
