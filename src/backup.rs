use anyhow::{bail, Context, Result};
use colored::*;
use std::{fs::copy, path::PathBuf};

pub fn backup(
    (existing_save_file_path, exists): &(PathBuf, bool),
    save_file_slot: usize,
) -> Result<()> {
    if !exists {
        bail!(
            "You cannot backup a save file for slot {} since the corresponding file does not exist.",
            save_file_slot,
        );
    }

    let dir_path = get_dir_of_running_exe()?;
    let file_name = existing_save_file_path.file_name().context(format!(
        "Failed to get the file name from the path of: {}",
        existing_save_file_path.display(),
    ))?;
    let destination_path = dir_path.join(file_name);

    let destination_path_string = destination_path.to_str().context(format!(
        "Failed to convert the path to a string: {}",
        destination_path.display(),
    ))?;
    if destination_path.exists() {
        bail!(
            "You cannot backup that save file because the following file already exists in the directory next to this program:\n{}",
            destination_path_string.green(),
        )
    }

    copy(&existing_save_file_path, &destination_path).context(format!(
        "Failed to copy {} --> {}",
        existing_save_file_path.display(),
        destination_path.display(),
    ))?;

    let existing_save_file_path_string = existing_save_file_path.to_str().context(format!(
        "Failed to convert the path to a string: {}",
        existing_save_file_path.display(),
    ))?;
    println!(
        "Successfully copied:\n{}\n-->\n{}",
        existing_save_file_path_string.green(),
        destination_path_string.green(),
    );

    Ok(())
}

fn get_dir_of_running_exe() -> Result<PathBuf> {
    let mut exe_path =
        std::env::current_exe().context("Failed to get the path of the current executable.")?;
    exe_path.pop();

    Ok(exe_path)
}
