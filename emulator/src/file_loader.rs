use byteorder::ReadBytesExt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, BufReader};
use std::path::Path;

const NES_FILE_MAGIC_BYES: [u8; 4] = ['N' as u8, 'E' as u8, 'S' as u8, 0x1A];
const PRG_UNIT_SIZE: u16 = 16;
const CHR_UNIT_SIZE: u16 = 8;

#[derive(thiserror::Error, Debug)]
pub enum NesRomReadError {
    #[error("file format not supported")]
    FileFormatNotSupported,

    #[error("missing magic bytes")]
    MissingMagicBytes,

    #[error("missing prg rom")]
    MissingPrgRom
}

pub enum Nes {
    Ines,
    Nes2
}

pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
    SingleScreen
}

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
    prg_rom: Vec<u8>,
    chr_rom: Option<Vec<u8>>,
    mapper: u8,
    play_choice_inst_rom: Option<Vec<u8>>,
    play_choice_10: Option<Vec<u8>>,
    title: Option<[u8; 128]>

}


struct Nes2Header {


}

pub struct Nes2 {
    header: Nes2Header,

}

impl Ines {
    fn header_from_file<F: AsRef<File>>(file: F) -> anyhow::Result<InesHeader> {

        let mut header = [0; 16];
        file.read_exact(&mut header)?;

        if header[0..4] != NES_FILE_MAGIC_BYES {
            return Err(NesRomReadError::MissingMagicBytes.into());
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
            zero
        })
    }
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Ines> {
        let mut file = BufReader::new(File::open(path.clone())?);
        let header = Ines::header_from_file(&path)?;

        let is_trainer_present = header.flags_6 & 0b00000100 != 0;

        let mut trainer = None;
        if is_trainer_present {
            let mut trainer_data = [0; 512];
            file.read_exact(&mut trainer_data)?;
            trainer = Some(trainer_data);
        }

        let prg_rom = Ines::read_banks(&mut file, header.prg_rom_size, PRG_UNIT_SIZE)?;
        let chr_rom = if header.chr_rom_size != 0 {
            Some(Ines::read_banks(&mut file, header.chr_rom_size, CHR_UNIT_SIZE)?)
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
            prg_rom,
            chr_rom,
            mapper,
            play_choice_inst_rom,
            play_choice_10,
            title
        })
    }

    fn read_banks(file: &mut BufReader<File>, bank_count: u8, unit_size: u16) -> anyhow::Result<Vec<u8>> {
        let mut banks = Vec::new();
        for _ in 0..bank_count {
            let mut bank = vec![0; unit_size as usize];
            file.read_exact(&mut bank)?;
            banks.append(&mut bank);
        }
        Ok(banks)
    }
}