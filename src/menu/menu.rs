use crate::menu::menu_item::MenuItem;

#[derive(Debug, Default)]
pub struct Menu {
    title: String,
    menu_items: std::vec::Vec<MenuItem>,
}

impl Menu {
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn add_menu_item(&mut self, item: MenuItem) {
        self.menu_items.push(item);
    }

    pub fn run(&self) {
        let mut running = true;
        while running {
            self.print_menu();
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();
            if input.is_empty() {
                println!("Invalid input");
                continue;
            }

            let input = input.parse::<usize>().unwrap() - 1;
            if input > self.menu_items.len() {
                println!("Invalid input");
                continue;
            }

            running &= (self.menu_items[input].command)();
        }
    }

    fn print_menu(&self) {
        println!("=====================");
        println!("{}", self.title);
        for i in 0..self.menu_items.len() {
            println!("{}. {}", i + 1, self.menu_items[i].name);
        }
        println!("=====================");
    }
}
