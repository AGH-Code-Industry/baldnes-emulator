use std::io::Read;
use std::path::Path;

pub const NES_FILE_MAGIC_BYTES: [u8; 4] = ['N' as u8, 'E' as u8, 'S' as u8, 0x1A];
pub const PRG_UNIT_SIZE: u16 = 16;
pub const CHR_UNIT_SIZE: u16 = 8;

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
    fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> where Self: Sized;
}

pub fn read_banks<R: Read>(file: &mut R, bank_count: u8, unit_size: u16) -> anyhow::Result<Vec<u8>> {
    let mut banks = Vec::new();
    for _ in 0..bank_count {
        let mut bank = vec![0; unit_size as usize];
        file.read_exact(&mut bank)?;
        banks.append(&mut bank);
    }
    Ok(banks)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_banks_2_4() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut reader = std::io::Cursor::new(data);
        let banks = read_banks(&mut reader, 2, 4).unwrap();
        assert_eq!(banks, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_read_banks_2_3() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06];
        let mut cursor = std::io::Cursor::new(data);
        let banks = read_banks(&mut cursor, 2, 3).unwrap();
        assert_eq!(banks, vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }
}