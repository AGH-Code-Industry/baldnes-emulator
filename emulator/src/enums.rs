#[derive(Debug, PartialEq)]
pub enum Nes {
    Ines,
    Nes2
}

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    SingleScreen,
    FourScreen
}