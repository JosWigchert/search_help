use clap_complete::{
    generate_to,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
};
use std::env;
use std::path::Path;

include!("src/cli.rs"); // Include the CLI definition from the main code

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir);

    // Create your CLI instance
    let mut cmd = cli();

    // Generate completion scripts for all supported shells
    generate_to(Bash, &mut cmd, "autocomplete-app", &dest_path).unwrap();
    println!("Generated completion script for {} shell", "Bash");

    generate_to(Zsh, &mut cmd, "autocomplete-app", &dest_path).unwrap();
    println!("Generated completion script for {} shell", "Zsh");

    generate_to(Fish, &mut cmd, "autocomplete-app", &dest_path).unwrap();
    println!("Generated completion script for {} shell", "Fish");

    generate_to(Elvish, &mut cmd, "autocomplete-app", &dest_path).unwrap();
    println!("Generated completion script for {} shell", "Elvish");

    generate_to(PowerShell, &mut cmd, "autocomplete-app", &dest_path).unwrap();
    println!("Generated completion script for {} shell", "PowerShell");
}
