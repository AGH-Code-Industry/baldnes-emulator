#[derive(thiserror::Error, Debug)]
pub enum NesRomReadError {
    #[error("file format not supported")]
    FileFormatNotSupported,

    #[error("missing magic bytes")]
    MissingMagicBytes,

    #[error("missing prg rom")]
    MissingPrgRom,
}
