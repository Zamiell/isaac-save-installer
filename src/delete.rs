use anyhow::{bail, Context, Result};
use camino::Utf8PathBuf;
use colored::*;
use std::fs::remove_file;

use crate::utils::get_dir_of_running_exe;

pub fn delete(
    (existing_save_file_path, exists): &(Utf8PathBuf, bool),
    save_file_slot: usize,
) -> Result<()> {
    if !exists {
        bail!(
            "You cannot delete a save file for slot {} since the corresponding file does not exist.",
            save_file_slot,
        );
    }

    let _dir_path = get_dir_of_running_exe()?;
    let _file_name = existing_save_file_path.file_name().context(format!(
        "Failed to get the file name from the path of:\n{}",
        existing_save_file_path.to_string().green(),
    ))?;

    remove_file(existing_save_file_path).context(format!(
        "Failed to delete:\n{}",
        existing_save_file_path.to_string().green(),
    ))?;

    println!(
        "Successfully deleted:\n{}",
        existing_save_file_path.to_string().green(),
    );

    Ok(())
}
