use crate::{COMPLETED_TEXT_FG_COLOR, TEXT_FG_COLOR};
use ratzilla::ratatui::prelude::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    Todo,
    Completed,
}

impl Status {
    pub const fn get_status_char(self) -> char {
        match self {
            Self::Todo => '☐',
            Self::Completed => '✓',
        }
    }

    pub const fn get_status_color(self) -> Color {
        match self {
            Self::Todo => TEXT_FG_COLOR,
            Self::Completed => COMPLETED_TEXT_FG_COLOR,
        }
    }
}
