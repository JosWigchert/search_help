mod choise;
mod cli;
mod command;
mod db;
mod list;

use choise::print_choice;
use clap::ArgMatches;
use colored::Colorize;
use command::get_command_texts;
use core::str;
use crossterm::{cursor, event, execute, terminal};
use db::{open_connection, program::Program};
use list::list_settings::ListIndicator;
use list::{list_item::ListItem, list_settings::ListSettings, print_list};
use std::io::{self, Write};

fn main() {
    let mut cli = cli::cli();
    let matches = cli.clone().get_matches();

    let conn = open_connection().expect("Error opening database");
    Program::create_table(&conn).expect("Error creating Program table");

    match matches.subcommand() {
        Some(("add", sub_m)) => add(sub_m),
        Some(("update", sub_m)) => update(sub_m),
        Some(("delete", sub_m)) => delete(sub_m),
        Some(("search", sub_m)) => search(sub_m),
        _ => cli.print_help().expect("Failed to print help"),
    }
}

fn add(sub_m: &ArgMatches) {
    let name = sub_m.get_one::<String>("name").expect("Name is required");
    let command = sub_m
        .get_one::<String>("command")
        .expect("Command is required");

    let command_strings = command.split(";").map(|c| c.trim()).collect::<Vec<&str>>();

    let conn = open_connection().expect("Error opening database");
    let mut programs = Program::get_all(&conn).expect("Error getting programs");

    let current_program_index = programs.iter().position(|p| p.name == *name);

    if let Some(index) = current_program_index {
        let program = &mut programs[index];
        let choice = print_choice(
            "Program with that name already exists, do you wish to update it?",
            vec!["y", "N", ""],
        );

        if let Ok(input) = choice {
            match input.as_str() {
                "y" => {
                    let texts = get_command_texts(&command_strings);
                    program.help_text = texts;
                    program.update(&conn).expect("Error updating program");
                    return;
                }
                _ => {
                    return;
                }
            }
        }
    }

    let texts = get_command_texts(&command_strings);

    let program = Program::new(
        name,
        command_strings
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        &texts,
    );
    let result = program.insert(&conn);

    match result {
        Ok(_) => {
            println!("Program added successfully");
        }
        Err(e) => {
            println!("Error adding program: {}", e);
        }
    }
}

fn update(sub_m: &ArgMatches) {
    let conn = open_connection().expect("Error opening database");

    let program = sub_m.get_one::<String>("name");

    match program {
        Some(name) => {
            let program = Program::get_by_name(&conn, name).expect("Error getting program");

            match program {
                Some(program) => {
                    update_program(&program);
                }
                None => {
                    println!("Program with that name does not exist");
                }
            }
        }
        None => {
            let programs = Program::get_all(&conn).expect("Error getting programs");

            let list_items = programs
                .iter()
                .map(|p| ListItem::new(p.name.as_str(), p, |p| update_program(*p)))
                .collect::<Vec<ListItem<&Program>>>();

            print_list(
                list_items,
                ListSettings::new(ListIndicator::Bullet, |s| s.yellow(), |s| s.blue()),
            )
            .expect("Error printing list");
        }
    }
}

fn update_program(item: &Program) -> bool {
    let conn = open_connection().expect("Error opening database");
    let program = Program::get_by_id(&conn, item.id.unwrap()).expect("Error getting program");
    if let Some(mut program) = program {
        let result = edit_program(&mut program);
        match result {
            Ok(true) => {
                program.update(&conn).expect("Error updating program");
            }
            Ok(false) => {
                println!("Program update canceled.");
            }
            Err(e) => {
                println!("Error editing program: {}", e);
            }
        }
    }

    false
}

enum EditField {
    Name,
    Commands,
}

fn edit_program(program: &mut Program) -> Result<bool, std::io::Error> {
    let mut current_field = EditField::Name;
    let mut cursor_pos = program.name.len(); // Start cursor at the end of the Name field
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;
    execute!(stdout, cursor::Show)?;
    execute!(stdout, cursor::SetCursorStyle::BlinkingBar)?;

    let mut commands = program.commands.join("; ");

    loop {
        // Clear the screen and display the fields
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        println!("Edit Program");
        execute!(stdout, cursor::MoveTo(0, 1))?;
        println!(
            "Name: {}{}",
            if let EditField::Name = current_field {
                "→ "
            } else {
                "  "
            },
            program.name
        );
        execute!(stdout, cursor::MoveTo(0, 2))?;
        println!(
            "Help Text: {}{}",
            if let EditField::Commands = current_field {
                "→ "
            } else {
                "  "
            },
            commands
        );

        // Move cursor to the right position
        match current_field {
            EditField::Name => execute!(stdout, cursor::MoveTo(8 + cursor_pos as u16, 1))?,
            EditField::Commands => execute!(stdout, cursor::MoveTo(13 + cursor_pos as u16, 2))?,
        }
        stdout.flush()?;

        match event::read()? {
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            })
            | event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            })
            | event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('x'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                // Cancel and exit without saving on Ctrl+C or Ctrl+X
                terminal::disable_raw_mode()?;
                execute!(stdout, terminal::LeaveAlternateScreen)?;
                return Ok(false); // Indicate canceled
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Tab,
                ..
            }) => {
                // Switch fields on Tab
                current_field = match current_field {
                    EditField::Name => EditField::Commands,
                    EditField::Commands => EditField::Name,
                };
                cursor_pos = match current_field {
                    EditField::Name => program.name.len(), // Move cursor to the end of the Name field
                    EditField::Commands => commands.len(), // Move cursor to the end of the Commands field
                };
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            }) => {
                // Insert characters at the current cursor position
                match current_field {
                    EditField::Name => {
                        program.name.insert(cursor_pos, c);
                        cursor_pos += 1;
                    }
                    EditField::Commands => {
                        commands.insert(cursor_pos, c);
                        cursor_pos += 1;
                    }
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Backspace,
                ..
            }) => {
                // Handle backspace to remove characters
                if cursor_pos > 0 {
                    match current_field {
                        EditField::Name => {
                            program.name.remove(cursor_pos - 1);
                        }
                        EditField::Commands => {
                            commands.remove(cursor_pos - 1);
                        }
                    }
                    cursor_pos -= 1;
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Delete,
                ..
            }) => {
                // Handle delete to remove characters at cursor position
                match current_field {
                    EditField::Name if cursor_pos < program.name.len() => {
                        program.name.remove(cursor_pos);
                    }
                    EditField::Commands if cursor_pos < commands.len() => {
                        commands.remove(cursor_pos);
                    }
                    _ => {}
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Left,
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                // Move cursor left by one word
                match current_field {
                    EditField::Name => {
                        if let Some(pos) = program.name[..cursor_pos].rfind(' ') {
                            cursor_pos = pos;
                        } else {
                            cursor_pos = 0; // Move to start if no space is found
                        }
                    }
                    EditField::Commands => {
                        if let Some(pos) = commands[..cursor_pos].rfind(';') {
                            cursor_pos = pos;
                        } else {
                            cursor_pos = 0; // Move to start if no semicolon is found
                        }
                    }
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Right,
                modifiers: event::KeyModifiers::CONTROL,
                ..
            }) => {
                // Move cursor right by one word
                match current_field {
                    EditField::Name => {
                        if let Some(pos) = program.name[cursor_pos..].find(' ') {
                            cursor_pos += pos + 1; // Move to the end of the next word
                        } else {
                            cursor_pos = program.name.len(); // Move to end if no space is found
                        }
                    }
                    EditField::Commands => {
                        if let Some(pos) = commands[cursor_pos..].find(';') {
                            cursor_pos += pos + 1; // Move to the end of the next command
                        } else {
                            cursor_pos = commands.len(); // Move to end if no semicolon is found
                        }
                    }
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Left,
                ..
            }) => {
                // Move cursor left
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Right,
                ..
            }) => {
                // Move cursor right
                match current_field {
                    EditField::Name if cursor_pos < program.name.len() => cursor_pos += 1,
                    EditField::Commands if cursor_pos < commands.len() => cursor_pos += 1,
                    _ => {}
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            }) => {
                // Exit and save on Enter
                break;
            }
            _ => {}
        }
    }

    // Disable raw mode and leave alternate screen on successful edit
    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen)?;

    program.commands = commands
        .split(';')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    Ok(true)
}

fn delete(sub_m: &ArgMatches) {
    let conn = open_connection().expect("Error opening database");

    let program = sub_m.get_one::<String>("name");

    match program {
        Some(name) => {
            let program = Program::get_by_name(&conn, name).expect("Error getting program");

            match program {
                Some(program) => {
                    Program::delete(&conn, program.id.unwrap()).expect("Error deleting program");
                }
                None => {
                    println!("Program with that name does not exist");
                }
            }
        }
        None => {
            let programs = Program::get_all(&conn).expect("Error getting programs");

            let list_items = programs
                .iter()
                .map(|p| ListItem::new(p.name.as_str(), p, |p| delete_from_list(*p)))
                .collect::<Vec<ListItem<&Program>>>();

            print_list(
                list_items,
                ListSettings::new(ListIndicator::Bullet, |s| s.yellow(), |s| s.blue()),
            )
            .expect("Error printing list");
        }
    }
}

fn delete_from_list(item: &Program) -> bool {
    let conn = open_connection().expect("Error opening database");
    Program::delete(&conn, item.id.unwrap()).expect("Error deleting program");
    true
}

fn search(sub_m: &ArgMatches) {
    let query = sub_m.get_one::<String>("query").expect("Query is required");
    println!("Searching for: {}", query);
}
