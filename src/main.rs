use dialoguer::{theme::ColorfulTheme, Select};
use cargo_metadata::{ CargoOpt, MetadataCommand };
use std::env;
use std::process::Command;
use std::os::unix::process::CommandExt;

fn main() {
    let mut cmd = MetadataCommand::new();
    cmd.features(CargoOpt::AllFeatures);
    cmd.no_deps();

    let metadata = cmd.exec().unwrap(); // if this fails, there is no reason to continue

    // read aviable targets that user can choose
    let targets: Vec<&str> = metadata
        .packages
        .iter()
        .map(|t| t.name.as_str())
        .collect();

    // show user selection prompt
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose bin target to run: ")
        .default(0)
        .items(&targets[..])
        .interact()
        .unwrap();
    let target = targets[selection];

    // run target via 'cargo run'
    println!("Running {} target via 'cargo run'!", target);
    Command::new("cargo")
        .arg("run")
        .arg("--bin") 
        .arg(target) 
        // change working directory to dir that program was called from
        .current_dir(env::current_dir().unwrap())
        /*
            Becouse of this, this program will not work on windows.
            why?: https://stackoverflow.com/a/53479765
            
            in summary: Execute 'cargo run' command and replace parent process
            with process of this command.
        */
        .exec(); 
}
