use crate::cartridge::registers::chr_rom::ChrRom;
use crate::cartridge::registers::prg_rom::PrgRom;

pub trait CartridgeData {
    fn prg_rom(&self) -> &PrgRom;
    fn chr_rom(&self) -> &ChrRom;
}
