use std::fmt::Debug;

pub enum Mirroring {
    Horizontal,
    Vertical,
}

impl PartialEq for Mirroring {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Mirroring::Horizontal, Mirroring::Horizontal)
                | (Mirroring::Vertical, Mirroring::Vertical)
        )
    }
}

impl Debug for Mirroring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mirroring::Horizontal => write!(f, "Mirroring::Horizontal"),
            Mirroring::Vertical => write!(f, "Mirroring::Vertical"),
        }
    }
}
