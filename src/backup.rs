use anyhow::{bail, Context, Result};
use camino::Utf8PathBuf;
use colored::*;
use std::fs::copy;

pub fn backup(
    (existing_save_file_path, exists): &(Utf8PathBuf, bool),
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
        "Failed to get the file name from the path of:\n{}",
        existing_save_file_path.to_string().green(),
    ))?;
    let destination_path = dir_path.join(file_name);

    if destination_path.exists() {
        bail!(
            "You cannot backup that save file because the following file already exists in the directory next to this program:\n{}",
            destination_path.to_string().green(),
        )
    }

    copy(&existing_save_file_path, &destination_path).context(format!(
        "Failed to copy:\n{}\n-->\n{}",
        existing_save_file_path.to_string().green(),
        destination_path.to_string().green(),
    ))?;

    println!(
        "Successfully copied:\n{}\n-->\n{}",
        existing_save_file_path.to_string().green(),
        destination_path.to_string().green(),
    );

    Ok(())
}

fn get_dir_of_running_exe() -> Result<Utf8PathBuf> {
    let exe_path =
        std::env::current_exe().context("Failed to get the path of the current executable.")?;
    let exe_path_utf8_result = Utf8PathBuf::from_path_buf(exe_path.clone());
    let exe_path_utf8 = match exe_path_utf8_result {
        Ok(path_buf) => path_buf,
        _ => bail!(format!(
            "Failed to convert the following path to UTF8:\n{}",
            exe_path.display(),
        )),
    };
    let dir_path = exe_path_utf8
        .parent()
        .context("Failed to get the parent directory of the current executable.")?;

    Ok(dir_path.to_path_buf())
}
