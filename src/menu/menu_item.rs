#[derive(Debug)]
pub struct MenuItem {
    pub name: String,
    pub command: fn() -> bool,
}
