use crate::{
    constants::STEAM_CLOUD_NAME,
    enums::{Activity, IsaacVersion},
};
use anyhow::{bail, Context, Result};
use std::ops::RangeInclusive;
use text_io::try_read;

const SELECTION_ERROR_MSG: &str = "That is not a valid selection.";
const INPUT_NUMBER_EXPLANATION_MSG: &str = "[Type the number and press enter.]";
const INPUT_BOOL_EXPLANATION_MSG: &str = "[Type y or n and press enter.]";

fn get_user_input_string() -> Result<String> {
    let input: String = try_read!("{}").context(SELECTION_ERROR_MSG)?;
    println!();

    let trimmed_input = input.trim().to_string();
    Ok(trimmed_input)
}

fn get_user_input_number() -> Result<usize> {
    let input = get_user_input_string()?;
    let number: usize = input
        .parse()
        .context(format!("Failed to convert \"{}\" to a number.", input))?;
    Ok(number)
}

pub fn prompt_for_isaac_version() -> Result<IsaacVersion> {
    println!("Which game do you want to manage the save files for?");
    println!("1) The Binding of Isaac: Rebirth");
    println!("2) The Binding of Isaac: Afterbirth");
    println!("3) The Binding of Isaac: Afterbirth+ (Vanilla through Booster Pack 4)");
    println!("4) The Binding of Isaac: Afterbirth+ (Booster Pack 5)");
    println!("5) The Binding of Isaac: Repentance");
    println!("{}", INPUT_NUMBER_EXPLANATION_MSG);

    let input = get_user_input_number()?;
    let enum_value = input - 1; // e.g. 1 corresponds to element 0
    let isaac_version = IsaacVersion::from_repr(enum_value).context(SELECTION_ERROR_MSG)?;

    Ok(isaac_version)
}

pub fn confirm_toggle_steam_cloud(steam_cloud_enabled: bool) -> Result<bool> {
    let verb = match steam_cloud_enabled {
        true => "on",
        false => "off",
    };
    println!(
        "Currently, the \"{}\" feature is turned {}.",
        STEAM_CLOUD_NAME, verb,
    );
    if steam_cloud_enabled {
        println!("Turning it off will make the game read from the save files in the \"Documents\" directory instead of in the \"Steam\" directory.");
        println!("Doing this is recommended since it makes managing your save files easier.");
        println!("You can also try doing this as a troubleshooting technique if you have previously installed save files and the game does not seem to be reading them.");
        println!();
        println!("Do you want to turn it off?")
    } else {
        println!("Turning it on will make the game read from the save files in the \"Steam\" directory instead of in the \"Documents\" directory.");
        println!();
        println!("Do you want to turn it on?")
    }
    println!("{}", INPUT_BOOL_EXPLANATION_MSG);

    let input = get_user_input_string()?;

    match input.as_str() {
        "y" => Ok(true),
        "n" => Ok(false),
        _ => bail!(SELECTION_ERROR_MSG),
    }
}

pub fn prompt_for_activity() -> Result<Activity> {
    println!("What do you want to do?");
    println!("1) Install a new fully-unlocked file.");
    println!("2) Backup an existing save file.");
    println!("3) Delete an existing save file.");
    println!("4) Change your \"SteamCloud\" setting in the \"options.ini\" file.");
    println!("{}", INPUT_NUMBER_EXPLANATION_MSG);

    let input = get_user_input_number()?;
    let enum_value = input - 1; // e.g. 1 corresponds to element 0
    let activity = Activity::from_repr(enum_value).context(SELECTION_ERROR_MSG)?;

    Ok(activity)
}

pub fn prompt_for_save_file_slot(activity: Activity) -> Result<usize> {
    let verb = match activity {
        Activity::Backup => "backup",
        Activity::Install => "install the fully-unlocked save file to",
        _ => "touch",
    };

    println!("Which save file do you want to {}?", verb);
    println!("1) Save slot 1");
    println!("2) Save slot 2");
    println!("3) Save slot 3");
    println!("{}", INPUT_NUMBER_EXPLANATION_MSG);

    let input = get_user_input_number()?;
    if RangeInclusive::new(1, 3).contains(&input) {
        return Ok(input);
    }

    bail!(SELECTION_ERROR_MSG)
}

pub fn prompt_for_user_to_hit_enter() -> Result<()> {
    get_user_input_string()?;
    Ok(())
}
