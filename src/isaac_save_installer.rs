use anyhow::{bail, Context, Result};
use colored::*;
use std::{ops::RangeInclusive, path::PathBuf};
use sysinfo::{System, SystemExt};
use text_io::try_read;

use crate::{
    backup::backup,
    enums::{Activity, IsaacVersion},
    install::install,
    save_data_path::{
        get_documents_save_data_path, get_steam_cloud_enabled, get_steam_save_data_path,
    },
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const INPUT_EXPLANATION_MSG: &str = "[Type the number and press enter.]";
const SELECTION_ERROR_MSG: &str = "That is not a valid selection.";

pub fn isaac_save_installer() -> Result<()> {
    print_banner();
    check_if_isaac_open()?;

    let isaac_version = prompt_user_for_isaac_version()?;
    let steam_save_data_path = get_steam_save_data_path(isaac_version)?;
    let documents_save_data_path = get_documents_save_data_path(isaac_version)?;
    let steam_cloud_enabled = get_steam_cloud_enabled(&documents_save_data_path)?;
    let save_data_path = match steam_cloud_enabled {
        true => steam_save_data_path,
        false => documents_save_data_path,
    };

    let existing_save_files =
        get_existing_save_files(isaac_version, save_data_path, steam_cloud_enabled);
    print_save_files(&existing_save_files);

    let activity = prompt_user_for_activity()?;
    let save_file_slot = prompt_user_for_save_file_slot(activity)?;
    let save_file_index = save_file_slot - 1;
    let save_file = existing_save_files.get(save_file_index).context(format!(
        "Failed to get the save file at index: {}",
        save_file_index
    ))?;

    match activity {
        Activity::Backup => backup(save_file, save_file_slot),
        Activity::Install => install(save_file, isaac_version),
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
    println!("{}", "https://discord.com/invite/0Sokdog3miAGKovs".green());
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

fn prompt_user_for_isaac_version() -> Result<IsaacVersion> {
    println!("Which game do you want to manage the save files for?");
    println!("1) The Binding of Isaac: Rebirth");
    println!("2) The Binding of Isaac: Afterbirth");
    println!("3) The Binding of Isaac: Afterbirth+ (Vanilla through Booster Pack 4)");
    println!("4) The Binding of Isaac: Afterbirth+ (Booster Pack 5)");
    println!("5) The Binding of Isaac: Repentance");
    println!("{}", INPUT_EXPLANATION_MSG);

    let input: usize = try_read!("{}\n").context(SELECTION_ERROR_MSG)?;
    println!();

    let enum_value = input - 1; // e.g. 1 corresponds to element 0
    let isaac_version = IsaacVersion::from_repr(enum_value).context(SELECTION_ERROR_MSG)?;

    Ok(isaac_version)
}

fn get_existing_save_files(
    isaac_version: IsaacVersion,
    save_data_path: PathBuf,
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
            true => save_file_path.to_str().unwrap_or("[unknown]").green(),
            false => "[empty]".cyan(),
        };
        println!("{}) {}", i + 1, value);
    }
    println!();
}

fn prompt_user_for_activity() -> Result<Activity> {
    println!("What do you want to do?");
    println!("1) Backup an existing save file.");
    println!("2) Install a new fully-unlocked file.");
    println!("{}", INPUT_EXPLANATION_MSG);

    let input: usize = try_read!("{}\n").context(SELECTION_ERROR_MSG)?;
    println!();

    let enum_value = input - 1; // e.g. 1 corresponds to element 0
    let activity = Activity::from_repr(enum_value).context(SELECTION_ERROR_MSG)?;

    Ok(activity)
}

fn prompt_user_for_save_file_slot(activity: Activity) -> Result<usize> {
    let verb = match activity {
        Activity::Backup => "backup",
        Activity::Install => "install the fully-unlocked save file to",
    };

    println!("Which save file do you want to {}?", verb);
    println!("1) Save slot 1");
    println!("2) Save slot 2");
    println!("3) Save slot 3");
    println!("{}", INPUT_EXPLANATION_MSG);

    let input: usize = try_read!("{}\n").context(SELECTION_ERROR_MSG)?;
    println!();

    if RangeInclusive::new(1, 3).contains(&input) {
        return Ok(input);
    }

    bail!(SELECTION_ERROR_MSG)
}
