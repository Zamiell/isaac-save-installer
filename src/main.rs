use anyhow::Error;
use isaac_save_installer::isaac_save_installer;

mod backup;
mod enums;
mod install;
mod isaac_save_installer;
mod save_data_path;
mod save_file_afterbirth;
mod save_file_afterbirth_plus;
mod save_file_afterbirth_plus_bp5;
mod save_file_rebirth;
mod save_file_repentance;

fn main() {
    match isaac_save_installer() {
        Ok(()) => quit(),
        Err(err) => error(&err),
    }
}

pub fn error(msg: &Error) -> ! {
    println!("Error: {}", msg);
    quit();
}

pub fn quit() -> ! {
    println!();
    dont_disappear::enter_to_continue::default();
    std::process::exit(1);
}
