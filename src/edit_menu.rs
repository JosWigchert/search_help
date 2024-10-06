use crate::menu::{menu::Menu, menu_item::MenuItem};

pub struct EditMenu {}

impl EditMenu {
    pub fn new() -> Self {
        let mut menu = Menu::default();
        menu.set_title("Edit Menu");
        menu.add_menu_item(MenuItem {
            name: "Add program".to_string(),
            command: || {
                println!("Adding program...");
                true
            },
        });
        menu.add_menu_item(MenuItem {
            name: "List programs".to_string(),
            command: || {
                println!("Listing programs...");
                true
            },
        });
        menu.add_menu_item(MenuItem {
            name: "Back".to_string(),
            command: || {
                println!("Back to main menu...");
                false
            },
        });

        menu.run();

        EditMenu {}
    }
}
