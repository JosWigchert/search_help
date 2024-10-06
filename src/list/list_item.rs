pub struct ListItem<'a, T> {
    pub text: &'a str,
    object: T,
    callback: fn(&T) -> bool,
}

impl<'a, T> ListItem<'a, T> {
    pub fn new(text: &'a str, object: T, callback: fn(&T) -> bool) -> Self {
        ListItem {
            text,
            object,
            callback,
        }
    }

    pub fn callback(&self) -> bool {
        (self.callback)(&self.object)
    }
}
