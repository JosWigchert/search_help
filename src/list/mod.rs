pub mod list_item;
pub mod list_settings;

use crate::list::{list_item::ListItem, list_settings::ListSettings};
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyModifiers},
    execute, terminal,
};
use std::io::{self, Write};

pub fn print_list<T>(items: Vec<ListItem<T>>, settings: ListSettings) -> std::io::Result<()> {
    let indicator = settings.list_indicator.as_str();

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;
    execute!(stdout, cursor::Hide)?;

    let mut selected = 0;

    loop {
        // Display the list with an indicator for the selected item
        for (i, item) in items.iter().enumerate() {
            execute!(stdout, cursor::MoveTo(0, i as u16 + 1))?;
            if i == selected {
                println!("{} {}", (settings.selected_color)(indicator), item.text);
            } else {
                println!("{} {}", (settings.unselected_color)(indicator), item.text);
            }
        }

        // Wait for a keypress
        if let event::Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < items.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Char('j') => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Char('k') => {
                    if selected < items.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    if selected < items.len() {
                        let item = &items[selected];
                        if item.callback() {
                            break;
                        }
                    }
                }
                KeyCode::Char('x') => {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                        break;
                    }
                }
                KeyCode::Char('c') => {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                        break;
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Backspace => {
                    break;
                }
                _ => {}
            }
        }

        // Flush output so the terminal shows the updates
        stdout.flush()?;
    }

    // Restore terminal settings
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;
    execute!(stdout, terminal::LeaveAlternateScreen)?;

    Ok(())
}
