use crate::cartridge::common::consts::NES_FILE_MAGIC_BYTES;
use crate::cartridge::common::consts::PRG_UNIT_SIZE;
use crate::cartridge::common::enums::errors::NesRomReadError;
use crate::cartridge::common::enums::mirroring::Mirroring;
use crate::cartridge::common::traits::cartridge_data::CartridgeData;
use crate::cartridge::common::traits::file_loadable::FileLoadable;
use crate::cartridge::common::utils::file::read_banks;
use crate::cartridge::registers::chr_ram::ChrRam;
use crate::cartridge::registers::chr_rom::ChrRom;
use crate::cartridge::registers::prg_ram::PrgRam;
use crate::cartridge::registers::prg_rom::PrgRom;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

// TODO: implement CartridgeData for Nes2
// TODO: implement FileLoadable for Nes2
// TODO: Fix the code
// TODO: Extended Console Type
// TODO: VS Unisystem
struct Nes2Header {
    prg_rom_size: u8,
    chr_rom_size: u8,
    flags_6: u8,
    flags_7: u8,
    mapper: u8,
    submapper: u8,
    prg_ram_size: u8,
    chr_ram_size: u8,
    cpu_ppu_timing_mode: u8,
    vs_unisystem: Option<u8>,
    extended_console_type: Option<u8>,
    misc_rom_count: u8,
    default_expansion_device: u8,
}

impl Debug for Nes2Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Nes2Header")
            .field("prg_rom_size", &self.prg_rom_size)
            .field("chr_rom_size", &self.chr_rom_size)
            .field("flags_6", &self.flags_6)
            .field("flags_7", &self.flags_7)
            .field("mapper", &self.mapper)
            .field("submapper", &self.submapper)
            .field("prg_ram_size", &self.prg_ram_size)
            .field("chr_ram_size", &self.chr_ram_size)
            .field("cpu_ppu_timing_mode", &self.cpu_ppu_timing_mode)
            .field("vs_unisystem", &self.vs_unisystem)
            .field("extended_console_type", &self.extended_console_type)
            .field("misc_rom_count", &self.misc_rom_count)
            .field("default_expansion_device", &self.default_expansion_device)
            .finish()
    }
}

pub struct Nes2 {
    header: Nes2Header,
    prg_rom: PrgRom,
    chr_rom: Option<ChrRom>,
    trainer: Option<[u8; 512]>,
    prg_ram: Option<PrgRam>,
    chr_ram: Option<ChrRam>,
    mirroring: Mirroring,
}

impl Debug for Nes2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Nes2")
            .field("header", &self.header)
            .field("prg_rom", &self.prg_rom)
            .field("chr_rom", &self.chr_rom)
            .field("trainer", &self.trainer)
            .field("prg_ram", &self.prg_ram)
            .field("chr_ram", &self.chr_ram)
            .finish()
    }
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

        let prg_rom_size = header[4];
        let chr_rom_size = header[5];
        let flags_6 = header[6];
        let flags_7 = header[7];
        let mapper = (flags_6 & 0xF0) | (flags_7 >> 4);
        let submapper = (flags_6 & 0x0F) | (flags_7 & 0x0F);
        let prg_ram_size = header[8];
        let chr_ram_size = header[9];
        let cpu_ppu_timing_mode = header[10];
        let vs_unisystem = if header[11] != 0 {
            Some(header[11])
        } else {
            None
        };
        let extended_console_type = if header[12] != 0 {
            Some(header[12])
        } else {
            None
        };
        let misc_rom_count = header[13];
        let default_expansion_device = header[14];

        Ok(Nes2Header {
            prg_rom_size,
            chr_rom_size,
            flags_6,
            flags_7,
            mapper,
            submapper,
            prg_ram_size,
            chr_ram_size,
            cpu_ppu_timing_mode,
            vs_unisystem,
            extended_console_type,
            misc_rom_count,
            default_expansion_device,
        })
    }
}

impl CartridgeData for Nes2 {
    fn prg_rom(&self) -> &PrgRom {
        &self.prg_rom
    }

    fn chr_rom(&self) -> &ChrRom {
        match self.chr_rom.as_ref() {
            Some(x) => x,
            None => panic!("CHR ROM is not present"),
        }
    }
}

impl FileLoadable for Nes2 {
    fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Nes2> {
        let mut file = BufReader::new(File::open(path)?);
        let header = Nes2::header_from_file(&mut file)?;

        let is_trainer_present = header.flags_6 & 0b00000100 != 0;

        let mirroring = if header.flags_6 & 0b00000001 != 0 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };

        let mut trainer = if is_trainer_present {
            let mut trainer_data = [0; 512];
            file.read_exact(&mut trainer_data)?;
            Some(trainer_data)
        } else {
            None
        };

        let prg_rom =
            PrgRom::new_with_data(read_banks(&mut file, header.prg_rom_size, PRG_UNIT_SIZE)?);

        let chr_rom = if header.chr_rom_size != 0 {
            Some(ChrRom::new_with_data(read_banks(
                &mut file,
                header.chr_rom_size,
                PRG_UNIT_SIZE,
            )?))
        } else {
            None
        };

        let prg_ram = if header.prg_ram_size != 0 {
            Some(PrgRam::new(header.prg_ram_size as usize))
        } else {
            None
        };

        let chr_ram = if header.chr_ram_size != 0 {
            Some(ChrRam::new(header.chr_ram_size as usize))
        } else {
            None
        };

        Ok(Nes2 {
            header,
            prg_rom,
            chr_rom,
            trainer,
            prg_ram,
            chr_ram,
            mirroring,
        })
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
