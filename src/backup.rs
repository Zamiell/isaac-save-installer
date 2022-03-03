use anyhow::{bail, Context, Result};
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

    if destination_path.exists() {
        bail!(
            "You cannot backup that save file because the following file already exists in the directory next to this program:\n{}",
            destination_path.display(),
        )
    }

    copy(&existing_save_file_path, &destination_path).context(format!(
        "Failed to copy {} --> {}",
        existing_save_file_path.display(),
        destination_path.display(),
    ))?;

    println!(
        "Successfully copied:\n{}\n-->\n{}",
        existing_save_file_path.display(),
        destination_path.display(),
    );

    Ok(())
}

fn get_dir_of_running_exe() -> Result<PathBuf> {
    let mut exe_path =
        std::env::current_exe().context("Failed to get the path of the current executable.")?;
    exe_path.pop();

    Ok(exe_path)
}
