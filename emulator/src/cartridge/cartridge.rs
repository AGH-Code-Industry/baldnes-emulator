use crate::cartridge::common::consts::NES_FILE_MAGIC_BYTES;
use crate::cartridge::common::enums::errors::NesRomReadError;
use crate::cartridge::common::enums::nes::Nes;
use crate::cartridge::file_loader::FileLoadable;
use crate::cartridge::formats::i_nes::Ines;
use crate::cartridge::formats::nes_2::Nes2;
use crate::cartridge::registers::chr_rom::ChrRom;
use crate::cartridge::registers::prg_rom::PrgRom;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

pub trait CartridgeData {
    fn prg_rom(&self) -> &PrgRom;
    fn chr_rom(&self) -> &ChrRom;
}

pub struct Cartridge {
    data: Box<dyn CartridgeData>,
}

impl Cartridge {
    // prepare cartridge with FileLoadable trait

    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Cartridge> {
        let mut file = BufReader::new(File::open(&path)?);
        let nes_type = Cartridge::nes_type_from_file(&mut file)?;
        // reset file pointer
        file.seek(SeekFrom::Start(0))?;
        match nes_type {
            Nes::Ines => {
                let ines = Ines::from_file(path)?;
                Ok(Cartridge {
                    data: Box::new(ines),
                })
            }
            Nes::Nes2 => {
                let nes2 = Nes2::from_file(path)?;
                Ok(Cartridge {
                    data: Box::new(nes2),
                })
            }
        }
    }

    fn nes_type_from_file<R: Read + Seek>(file: &mut R) -> anyhow::Result<Nes> {
        let mut header = [0; 16];
        file.read_exact(&mut header)?;
        // Is it a NES file?
        if header[0..4] != NES_FILE_MAGIC_BYTES {
            return Err(NesRomReadError::MissingMagicBytes.into());
        }
        // NES 2.0
        if (header[7] & 0x0C) == 0x08 {
            // reset file pointer
            file.seek(SeekFrom::Start(0))?;
            return Ok(Nes::Nes2);
        }
        Ok(Nes::Ines)
    }
}

impl CartridgeData for Cartridge {
    fn prg_rom(&self) -> &PrgRom {
        self.data.prg_rom()
    }

    fn chr_rom(&self) -> &ChrRom {
        self.data.chr_rom()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::common::consts::{CHR_UNIT_SIZE, PRG_UNIT_SIZE};

    #[test]
    fn test_from_file() {
        // Super Mario Bros
        // check if the file is in the resources folder
        let is_file = std::path::Path::new("resources/smb.nes").exists();
        // issue a warning if the file is not found
        if !is_file {
            println!("resources/smb.nes not found");
            return;
        }
        let cartridge = Cartridge::from_file("resources/smb.nes");
        assert!(cartridge.is_ok());
        let cartridge = cartridge.unwrap();

        let prg_rom = cartridge.prg_rom();

        let chr_rom = cartridge.chr_rom();

        assert_eq!(prg_rom.size(), 2 * PRG_UNIT_SIZE as usize);
        assert_eq!(chr_rom.size(), 1 * CHR_UNIT_SIZE as usize);
    }
}
