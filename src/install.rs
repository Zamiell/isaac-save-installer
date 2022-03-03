use crate::{
    enums::IsaacVersion,
    save_files::{
        SAVE_FILE_AFTERBIRTH, SAVE_FILE_AFTERBIRTH_PLUS, SAVE_FILE_AFTERBIRTH_PLUS_BP5,
        SAVE_FILE_REBIRTH, SAVE_FILE_REPENTANCE,
    },
};
use anyhow::{Context, Result};
use std::{fs::write, path::PathBuf};

pub fn install(
    (save_file_path, _exists): &(PathBuf, bool),
    isaac_version: IsaacVersion,
) -> Result<()> {
    let save_file_bytes = get_save_file_bytes(isaac_version);

    write(save_file_path, save_file_bytes).context(format!(
        "Failed to write data to the following path: {}",
        save_file_path.display(),
    ))?;

    println!(
        "Successfully installed a fully-unlocked save file to:\n{}",
        save_file_path.display(),
    );

    Ok(())
}

fn get_save_file_bytes(isaac_version: IsaacVersion) -> &'static [u8] {
    match isaac_version {
        IsaacVersion::Rebirth => SAVE_FILE_REBIRTH,
        IsaacVersion::Afterbirth => SAVE_FILE_AFTERBIRTH,
        IsaacVersion::AfterbirthPlus => SAVE_FILE_AFTERBIRTH_PLUS,
        IsaacVersion::AfterbirthPlusBP5 => SAVE_FILE_AFTERBIRTH_PLUS_BP5,
        IsaacVersion::Repentance => SAVE_FILE_REPENTANCE,
    }
}
