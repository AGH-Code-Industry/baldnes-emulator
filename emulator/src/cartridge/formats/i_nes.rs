use crate::cartridge::cartridge::CartridgeData;
use crate::cartridge::common::enums::mirroring::Mirroring;
use crate::cartridge::file_loader::read_banks;
use crate::cartridge::file_loader::FileLoadable;
use crate::cartridge::registers::chr_rom::ChrRom;
use crate::cartridge::registers::prg_rom::PrgRom;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::cartridge::common::consts::{CHR_UNIT_SIZE, NES_FILE_MAGIC_BYTES, PRG_UNIT_SIZE};
use crate::cartridge::common::enums::errors::NesRomReadError;
use std::fmt::Debug;

// Bytes 	Description
// 0-3 	Constant $4E $45 $53 $1A (ASCII "NES" followed by MS-DOS end-of-file)
// 4 	Size of PRG ROM in 16 KB units
// 5 	Size of CHR ROM in 8 KB units (value 0 means the board uses CHR RAM)
// 6 	Flags 6 – Mapper, mirroring, battery, trainer
// 7 	Flags 7 – Mapper, VS/Playchoice, NES 2.0
// 8 	Flags 8 – PRG-RAM size (rarely used extension)
// 9 	Flags 9 – TV system (rarely used extension)
// 10 	Flags 10 – TV system, PRG-RAM presence (unofficial, rarely used extension)
// 11-15 	Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)
struct InesHeader {
    prg_rom_size: u8,
    chr_rom_size: u8,
    flags_6: u8,
    flags_7: u8,
    prg_ram_size: u8, // flags_8
    flags_9: u8,
    flags_10: u8,
    zero: [u8; 5],
}

impl Debug for InesHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InesHeader")
            .field("prg_rom_size", &self.prg_rom_size)
            .field("chr_rom_size", &self.chr_rom_size)
            .field("flags_6", &self.flags_6)
            .field("flags_7", &self.flags_7)
            .field("prg_ram_size", &self.prg_ram_size)
            .field("flags_9", &self.flags_9)
            .field("flags_10", &self.flags_10)
            .field("zero", &self.zero)
            .finish()
    }
}

// Header (16 bytes)
// Trainer, if present (0 or 512 bytes)
// PRG ROM data (16384 * x bytes)
// CHR ROM data, if present (8192 * y bytes)
// PlayChoice INST-ROM, if present (0 or 8192 bytes)
// PlayChoice PROM, if present (16 bytes Data, 16 bytes CounterOut) (this is often missing; see PC10 ROM-Images for details)
// Some ROM-Images additionally contain a 128-byte (or sometimes 127-byte) title at the end of the file.
pub struct Ines {
    header: InesHeader,
    trainer: Option<[u8; 512]>,
    mirroring: Mirroring,
    battery: bool,
    four_screen_vram: bool,
    prg_rom: PrgRom,
    chr_rom: Option<ChrRom>,
    mapper: u8,
    play_choice_inst_rom: Option<Vec<u8>>,
    play_choice_10: Option<Vec<u8>>,
    title: Option<[u8; 128]>,
}

impl Debug for Ines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ines")
            .field("header", &self.header)
            .field("trainer", &self.trainer)
            .field("mirroring", &self.mirroring)
            .field("battery", &self.battery)
            .field("four_screen_vram", &self.four_screen_vram)
            .field("prg_rom", &self.prg_rom)
            .field("chr_rom", &self.chr_rom)
            .field("mapper", &self.mapper)
            .field("play_choice_inst_rom", &self.play_choice_inst_rom)
            .field("play_choice_10", &self.play_choice_10)
            .field("title", &self.title)
            .finish()
    }
}

impl Ines {
    fn header_from_file<R: Read>(file: &mut R) -> anyhow::Result<InesHeader> {
        let mut header = [0; 16];
        file.read_exact(&mut header)?;

        if header[0..4] != NES_FILE_MAGIC_BYTES {
            return Err(NesRomReadError::MissingMagicBytes.into());
        }
        // NES 2.0
        if (header[7] & 0x0C) == 0x08 {
            return Err(NesRomReadError::FileFormatNotSupported.into());
        }

        let prg_rom_size = header[4];
        let chr_rom_size = header[5];
        let flags_6 = header[6];
        let flags_7 = header[7];
        let prg_ram_size = header[8];
        let flags_9 = header[9];
        let flags_10 = header[10];
        let zero = [header[11], header[12], header[13], header[14], header[15]];

        Ok(InesHeader {
            prg_rom_size,
            chr_rom_size,
            flags_6,
            flags_7,
            prg_ram_size,
            flags_9,
            flags_10,
            zero,
        })
    }
}

impl FileLoadable for Ines {
    fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Ines> {
        let mut file = BufReader::new(File::open(path)?);
        let header = Ines::header_from_file(&mut file)?;

        let is_trainer_present = header.flags_6 & 0b00000100 != 0;

        let mirroring = if header.flags_6 & 0b00000001 != 0 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };

        let battery = header.flags_6 & 0b00000010 != 0;

        let mut trainer = None;
        if is_trainer_present {
            let mut trainer_data = [0; 512];
            file.read_exact(&mut trainer_data)?;
            trainer = Some(trainer_data);
        }

        let four_screen_vram = header.flags_6 & 0b00001000 != 0;

        let prg_rom =
            PrgRom::new_with_data(read_banks(&mut file, header.prg_rom_size, PRG_UNIT_SIZE)?);

        let chr_rom = if header.chr_rom_size != 0 {
            Some(ChrRom::new_with_data(read_banks(
                &mut file,
                header.chr_rom_size,
                CHR_UNIT_SIZE,
            )?))
        } else {
            None
        };

        let mapper = (header.flags_6 & 0xF0) | (header.flags_7 & 0xF0);

        let play_choice_inst_rom = None;

        let play_choice_10 = None;
        let title = None;

        Ok(Ines {
            header,
            trainer,
            mirroring,
            battery,
            four_screen_vram,
            prg_rom,
            chr_rom,
            mapper,
            play_choice_inst_rom,
            play_choice_10,
            title,
        })
    }
}

impl CartridgeData for Ines {
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::formats::i_nes::FileLoadable;
    use std::io::Cursor;

    #[test]
    fn test_header_from_file() {
        let data = [
            0x4E, 0x45, 0x53, 0x1A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
            0x0B, 0x0C,
        ];
        let mut cursor = Cursor::new(data);
        let header = Ines::header_from_file(&mut cursor);
        assert!(header.is_ok());
        let header = header.unwrap();
        assert_eq!(header.prg_rom_size, 0x01);
        assert_eq!(header.chr_rom_size, 0x02);
        assert_eq!(header.flags_6, 0x03);
        assert_eq!(header.flags_7, 0x04);
        assert_eq!(header.prg_ram_size, 0x05);
        assert_eq!(header.flags_9, 0x06);
        assert_eq!(header.flags_10, 0x07);
        assert_eq!(header.zero, [0x08, 0x09, 0x0A, 0x0B, 0x0C]);
    }
    #[test]
    fn test_bad_header_from_file() {
        let data = [
            0x4E, 0x45, 0x53, 0x1A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A,
            0x0B,
        ];
        let mut cursor = Cursor::new(data);
        let header = Ines::header_from_file(&mut cursor);
        assert!(header.is_err());
    }

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
        let ines = Ines::from_file("resources/smb.nes").unwrap();

        // mapper
        assert_eq!(ines.mapper, 0);
        // mirroring
        assert_eq!(ines.mirroring, Mirroring::Vertical);
        // battery
        assert_eq!(ines.battery, false);

        // prg_rom
        // inary operation `==` cannot be applied to type `usize`
        assert_eq!(ines.prg_rom.size(), 2 * PRG_UNIT_SIZE as usize);
        assert_eq!(ines.header.prg_rom_size, 2);

        // chr_rom
        assert_eq!(
            ines.chr_rom.as_ref().unwrap().size(),
            1 * CHR_UNIT_SIZE as usize
        );
        assert_eq!(ines.header.chr_rom_size, 1);

        // trainer
        assert_eq!(ines.trainer.is_none(), true);

        println!("{:?}", ines);
    }
}
