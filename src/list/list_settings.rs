use colored::ColoredString;

pub enum ListIndicator {
    NoIndent,
    Indent,
    Bullet,
    Arrow,
}

impl ListIndicator {
    pub fn as_str(&self) -> &'static str {
        match self {
            ListIndicator::NoIndent => "",
            ListIndicator::Indent => "  ",
            ListIndicator::Bullet => "•",
            ListIndicator::Arrow => "→",
        }
    }
}

pub struct ListSettings {
    pub list_indicator: ListIndicator,
    pub selected_color: fn(&str) -> ColoredString,
    pub unselected_color: fn(&str) -> ColoredString,
}

impl ListSettings {
    pub fn new(
        list_indicator: ListIndicator,
        selected_color: fn(&str) -> ColoredString,
        unselected_color: fn(&str) -> ColoredString,
    ) -> Self {
        Self {
            list_indicator,
            selected_color,
            unselected_color,
        }
    }
}
