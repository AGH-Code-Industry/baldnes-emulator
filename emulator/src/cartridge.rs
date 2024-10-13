use byteorder::ReadBytesExt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub struct Cartridge {
    trainer: Option<[u8; 512]>,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    misc_area: Vec<u8>,
}

const NES_FILE_MAGIC_BYES: [u8; 4] = ['N' as u8, 'E' as u8, 'S' as u8, 0x1A];
const PRG_UNIT_SIZE: u16 = 16;
const CHR_UNIT_SIZE: u16 = 8;

#[derive(thiserror::Error, Debug)]
pub enum NesRomReadError {
    #[error("file format not supported")]
    FileFormatNotSupported
}

impl Cartridge {
    pub fn from_nes_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Cartridge> {
        let mut file = File::open(path)?;

        Self::read_check_is_valid_nes_file(&mut file)?;

        let prg_lsb = file.read_u8()?;
        let chr_lsb = file.read_u8()?;

        let flags_6 = file.read_u8()?;

        let is_trainer_present = flags_6 & 0b00000100 != 0;

        file.seek(SeekFrom::Current(2))?;

        let msbs = file.read_u8()?;

        let prg_msb = msbs & 0b00001111;
        let chr_msb = (msbs & 0b11110000) >> 4;

        file.seek(SeekFrom::Current(6))?;

        let trainer = if is_trainer_present {
            let mut trainer = [0; 512];
            file.read_exact(&mut trainer)?;
            Some(trainer)
        } else {
            None
        };

        let prg_rom = Self::read_banks(&mut file, prg_lsb, prg_msb, PRG_UNIT_SIZE)?;
        let chr_rom = Self::read_banks(&mut file, chr_lsb, chr_msb, CHR_UNIT_SIZE)?;

        let mut misc_area = Vec::new();
        file.read_to_end(misc_area.as_mut())?;

        Ok(Cartridge { trainer, prg_rom, chr_rom, misc_area })
    }

    fn read_check_is_valid_nes_file<R: Read>(file: &mut R) -> anyhow::Result<()> {
        let mut magic_bytes = [0; 4];
        file.read_exact(&mut magic_bytes)?;
        if (magic_bytes) != NES_FILE_MAGIC_BYES {
            return Err(anyhow::Error::new(NesRomReadError::FileFormatNotSupported));
        }
        Ok(())
    }

    fn read_banks<R: Read>(file: &mut R, lsb: u8, msb: u8, unit_size: u16) -> anyhow::Result<Vec<u8>> {
        let size = if msb != 0xF {
            (((msb as u16) << 8) | (lsb as u16)) * (unit_size * 1024)
        } else {
            let multiplier = (lsb & 0b00000011) as u16;
            let exponent = ((lsb & 0b11111100) >> 2) as u16;
            (1 << exponent) * (multiplier * 2 + 1)
        };
        let mut buf = vec![0; size as usize];
        file.read_exact(&mut buf)?;
        Ok(buf)
    }

    pub fn info(&self) -> String {
        let mut info = String::new();

        info.push_str("Trainer: ");
        match self.trainer {
            Some(_) => info.push_str("present\n"),
            None => info.push_str("not present\n")
        }

        info.push_str(format!("PRG ROM size: {}\n", self.prg_rom.len()).as_str());
        info.push_str(format!("CHR ROM size: {}\n", self.chr_rom.len()).as_str());
        info.push_str(format!("Misc ROM size: {}\n", self.misc_area.len()).as_str());

        info
    }
}