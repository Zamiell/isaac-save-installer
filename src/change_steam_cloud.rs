use crate::{get_input::confirm_toggle_steam_cloud, save_data_path::toggle_steam_cloud_enabled};
use anyhow::Result;
use camino::Utf8Path;

pub fn change_steam_cloud(
    documents_save_data_path: &Utf8Path,
    steam_cloud_enabled: bool,
) -> Result<()> {
    let confirm = confirm_toggle_steam_cloud(steam_cloud_enabled)?;
    if !confirm {
        return Ok(());
    }

    toggle_steam_cloud_enabled(documents_save_data_path, steam_cloud_enabled)?;

    Ok(())
}
