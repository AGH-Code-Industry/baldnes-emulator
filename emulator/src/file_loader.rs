use std::path::Path;

pub(crate) const NES_FILE_MAGIC_BYTES: [u8; 4] = ['N' as u8, 'E' as u8, 'S' as u8, 0x1A];
pub(crate) const PRG_UNIT_SIZE: u16 = 16;
pub(crate) const CHR_UNIT_SIZE: u16 = 8;

#[derive(thiserror::Error, Debug)]
pub enum NesRomReadError {
    #[error("file format not supported")]
    FileFormatNotSupported,

    #[error("missing magic bytes")]
    MissingMagicBytes,

    #[error("missing prg rom")]
    MissingPrgRom
}

pub trait FileLoadable {
    fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self>;
}
