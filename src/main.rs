use anyhow::Error;

use colored::*;
use get_input::prompt_for_user_to_hit_enter;
use isaac_save_installer::isaac_save_installer;

mod backup;
mod change_steam_cloud;
mod constants;
mod delete;
mod enums;
mod get_input;
mod install;
mod isaac_save_installer;
mod save_data_path;
mod save_files;
mod utils;

fn main() {
    match isaac_save_installer() {
        Ok(()) => quit(false),
        Err(err) => error(&err),
    }
}

pub fn error(msg: &Error) -> ! {
    println!("{} {}", "Error:".red(), msg);
    quit(true);
}

pub fn quit(errored: bool) -> ! {
    println!("You can now close this window.");
    prompt_for_user_to_hit_enter().ok();

    let exit_code = match errored {
        true => 1,
        false => 0,
    };
    std::process::exit(exit_code);
}
