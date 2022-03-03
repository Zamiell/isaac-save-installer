use anyhow::{Context, Result};
use colored::Colorize;
use std::{fs::write, path::PathBuf};

use crate::{
    enums::IsaacVersion, save_file_afterbirth::SAVE_FILE_AFTERBIRTH,
    save_file_afterbirth_plus::SAVE_FILE_AFTERBIRTH_PLUS,
    save_file_afterbirth_plus_bp5::SAVE_FILE_AFTERBIRTH_PLUS_BP5,
    save_file_rebirth::SAVE_FILE_REBIRTH, save_file_repentance::SAVE_FILE_REPENTANCE,
};

pub fn install(
    (save_file_path, _exists): &(PathBuf, bool),
    isaac_version: IsaacVersion,
) -> Result<()> {
    let save_file_base_64 = get_save_file_base_64(isaac_version);
    let save_file_data = base64::decode(save_file_base_64).context(format!(
        "Failed to decode the base 64 for the save file of: {}",
        isaac_version,
    ))?;

    write(save_file_path, save_file_data).context(format!(
        "Failed to write data to the following path: {}",
        save_file_path.display(),
    ))?;

    let save_file_path_string = save_file_path.to_str().context(format!(
        "Failed to convert the path to a string: {}",
        save_file_path.display(),
    ))?;
    println!(
        "Successfully installed a fully-unlocked save file to:\n{}",
        save_file_path_string.green(),
    );

    Ok(())
}

fn get_save_file_base_64(isaac_version: IsaacVersion) -> String {
    let save_file_base_64 = match isaac_version {
        IsaacVersion::Rebirth => SAVE_FILE_REBIRTH,
        IsaacVersion::Afterbirth => SAVE_FILE_AFTERBIRTH,
        IsaacVersion::AfterbirthPlus => SAVE_FILE_AFTERBIRTH_PLUS,
        IsaacVersion::AfterbirthPlusBP5 => SAVE_FILE_AFTERBIRTH_PLUS_BP5,
        IsaacVersion::Repentance => SAVE_FILE_REPENTANCE,
    };

    String::from(save_file_base_64)
}
