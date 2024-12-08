use std::fmt::Debug;

pub enum Nes {
    Ines,
    Nes2,
}

impl Debug for Nes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nes::Ines => write!(f, "Ines"),
            Nes::Nes2 => write!(f, "Nes2"),
        }
    }
}

impl PartialEq for Nes {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Nes::Ines, Nes::Ines) | (Nes::Nes2, Nes::Nes2)
        )
    }
}
