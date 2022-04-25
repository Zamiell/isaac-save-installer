use anyhow::{bail, Context, Result};
use camino::Utf8PathBuf;

pub fn get_dir_of_running_exe() -> Result<Utf8PathBuf> {
    let exe_path =
        std::env::current_exe().context("Failed to get the path of the current executable.")?;
    let exe_path_utf8_result = Utf8PathBuf::from_path_buf(exe_path);
    let exe_path_utf8 = match exe_path_utf8_result {
        Ok(path_buf) => path_buf,
        Err(path_buf) => bail!(format!(
            "Failed to convert the following path to UTF8:\n{:?}",
            path_buf,
        )),
    };
    let dir_path = exe_path_utf8
        .parent()
        .context("Failed to get the parent directory of the current executable.")?;

    Ok(dir_path.to_path_buf())
}
