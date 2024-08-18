use dialoguer::{theme::ColorfulTheme, Select};
use cargo_metadata::{ CargoOpt, MetadataCommand };

fn main() {
    let mut cmd = MetadataCommand::new();
    cmd.features(CargoOpt::AllFeatures);
    cmd.no_deps();

    let metadata = cmd.exec().unwrap(); // if this fails, there is no reason to continue

    let targets: Vec<&str> = metadata
        .packages
        .iter()
        .map(|t| t.name.as_str())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose bin target to run: ")
        .default(0)
        .items(&targets[..])
        .interact()
        .unwrap();

    println!("Running {} target via 'cargo run'!", targets[selection]);
 
}
