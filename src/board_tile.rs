use std::fmt;

pub struct BoardTile {
    pub value: char,
    pub initialized: bool,
}

impl fmt::Display for BoardTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.value)
    }
}
