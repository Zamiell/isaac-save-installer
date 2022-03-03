use anyhow::Error;
use colored::*;

const ERROR_PREFIX: &str = "Error:";

pub fn get_username() -> String {
    whoami::username()
}

pub fn error(msg: &Error) -> ! {
    let prefix = ERROR_PREFIX.red();
    println!("{prefix} {msg}");
    println!();
    quit();
}

pub fn quit() -> ! {
    dont_disappear::enter_to_continue::default();
    std::process::exit(1);
}
