use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Pawn {
    Red,
    Blue,
    White,
}

impl fmt::Display for Pawn {
    /// Display the pawn as a unicode character.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pawn::Red => write!(f, "🔴"),
            Pawn::Blue => write!(f, "🔵"),
            Pawn::White => write!(f, "⚪"),
        }
    }
}

impl Pawn {
    /// Switch the color of the pawn, red to blue, blue to red.
    pub fn switch(&mut self) {
        *self = match self {
            Pawn::Red => Pawn::Blue,
            Pawn::Blue => Pawn::Red,
            Pawn::White => Pawn::White,
        }
    }
}
