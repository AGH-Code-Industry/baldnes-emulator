use std::fmt::Debug;

pub enum Mirroring {
    Horizontal,
    Vertical,
}

impl PartialEq for Mirroring {
    fn eq(&self, other: &Self) -> bool {
        if let (Mirroring::Horizontal, Mirroring::Horizontal) = (self, other) {
            true
        } else if let (Mirroring::Vertical, Mirroring::Vertical) = (self, other) {
            true
        } else {
            false
        }
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
