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
pub enum Mirroring {
    Horizontal,
    Vertical,
    SingleScreen,
    FourScreen,
}

impl Debug for Mirroring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mirroring::Horizontal => write!(f, "Mirroring::Horizontal"),
            Mirroring::Vertical => write!(f, "Mirroring::Vertical"),
            Mirroring::SingleScreen => write!(f, "Mirroring::SingleScreen"),
            Mirroring::FourScreen => write!(f, "Mirroring::FourScreen"),
        }
    }
}

impl PartialEq for Mirroring {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Mirroring::Horizontal, Mirroring::Horizontal)
                | (Mirroring::Vertical, Mirroring::Vertical)
                | (Mirroring::SingleScreen, Mirroring::SingleScreen)
                | (Mirroring::FourScreen, Mirroring::FourScreen)
        )
    }
}
