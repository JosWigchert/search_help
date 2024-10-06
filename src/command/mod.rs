pub mod errors;

use errors::GetCommandStringError;
use std::process::Command;

pub fn get_command_texts(command_strings: &Vec<&str>) -> String {
    let mut texts: String = String::new();

    for command in command_strings {
        let output = get_command_text(command);
        match output {
            Ok(output) => {
                texts.push_str(&output);
                texts.push('\n');
            }
            Err(e) => println!("Error when executing command: \"{}\": {}", command, e),
        }
    }

    texts
}

pub fn get_command_text(command: &str) -> Result<String, GetCommandStringError> {
    let main_command = command.split_whitespace().next().unwrap().trim();
    let command_args = command.split_whitespace().skip(1).collect::<Vec<&str>>();

    let output = Command::new(main_command).args(command_args).output()?;
    let output_str = String::from_utf8(output.stdout)?;
    Ok(output_str)
}
