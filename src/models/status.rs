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
}