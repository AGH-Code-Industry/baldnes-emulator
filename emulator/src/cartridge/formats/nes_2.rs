use crate::cartridge::cartridge::CartridgeData;
use crate::cartridge::file_loader::{FileLoadable, NesRomReadError, NES_FILE_MAGIC_BYTES};
use crate::cartridge::registers::chr_rom::ChrRom;
use crate::cartridge::registers::prg_rom::PrgRom;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

struct Nes2Header {}
#[allow(dead_code)]
pub struct Nes2 {
    header: Nes2Header,
}
impl Nes2 {
    fn header_from_file<R: Read>(file: &mut R) -> anyhow::Result<Nes2Header> {
        let mut header = [0; 16];
        file.read_exact(&mut header)?;

        if header[0..4] != NES_FILE_MAGIC_BYTES {
            return Err(NesRomReadError::MissingMagicBytes.into());
        }
        // NES 2.0
        if (header[7] & 0x0C) != 0x08 {
            return Err(NesRomReadError::FileFormatNotSupported.into());
        }

        Ok(Nes2Header {})
    }
}

impl CartridgeData for Nes2 {
    fn prg_rom(&self) -> &PrgRom {
        unimplemented!()
    }

    fn chr_rom(&self) -> &ChrRom {
        unimplemented!()
    }
}

impl FileLoadable for Nes2 {
    fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Nes2> {
        let mut file = BufReader::new(File::open(path)?);
        let header = Nes2::header_from_file(&mut file)?;

        Ok(Nes2 { header })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_from_file() {
        let data = [
            'N' as u8, 'E' as u8, 'S' as u8, 0x1A, 0, 0, 0, 0x08, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut cursor = std::io::Cursor::new(data);
        let header = Nes2::header_from_file(&mut cursor).unwrap();
    }
}
