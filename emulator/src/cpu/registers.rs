use crate::bus::BusLike;
use crate::cpu::cpu::CPUFlag;
use crate::cpu::micro_instructions::MicroInstructionSequence;
use crate::cpu::operations::Operation;

pub struct Registers {
    pub x: u8,
    pub y: u8,
    pub a: u8,
    program_counter: u16,
    stack_ptr: u8,
    status: u8,
    operation: u8,
    adl: u8,
    adh: u8,
    bal: u8,
    bah: u8,
    ial: u8,
    decoded_addressing_mode: Option<MicroInstructionSequence>,
    decoded_operation: Option<MicroInstructionSequence>,
    pub memory_buffer: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            x: 0x00,
            y: 0x00,
            a: 0x00,
            program_counter: 0x0000,
            stack_ptr: 0x00,
            status: 0x00,
            operation: 0x00,
            adl: 0x00,
            adh: 0x00,
            bal: 0x00,
            bah: 0x00,
            ial: 0x00,
            decoded_addressing_mode: None,
            decoded_operation: None,
            memory_buffer: 0x00,
        }
    }

    pub fn get_operation(&mut self) -> &mut Option<MicroInstructionSequence> {
        match self.decoded_addressing_mode {
            Some(ref mut decoded_addressing_mode) => {
                if decoded_addressing_mode.is_completed() {
                    &mut self.decoded_operation
                } else {
                    &mut self.decoded_addressing_mode
                }
            }
            None => &mut self.decoded_operation,
        }
    }

    pub fn is_operation_completed(&self) -> bool {
        match &self.decoded_operation {
            Some(operation) => operation.is_completed(),
            None => false,
        }
    }

    pub fn set_flag(&mut self, flag: CPUFlag) {
        self.status |= flag.value();
    }

    pub fn clear_flag(&mut self, flag: CPUFlag) {
        self.status &= !flag.value();
    }

    pub fn set_flag_value(&mut self, flag: CPUFlag, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }

    pub fn is_flag_set(&self, flag: CPUFlag) -> bool {
        self.status & flag.value() != 0
    }

    pub fn reset_flags(&mut self) {
        self.status = 0x00;
    }

    pub fn step_program_counter(&mut self) {
        self.program_counter += 1;
    }

    pub fn read_operation_code<T: BusLike>(&mut self, bus: &mut T) {
        self.operation = bus.read(self.program_counter as u16);
    }

    pub fn decode_operation<T: BusLike>(&mut self, bus: &T) {
        let operation_code = self.operation;
        println!("Operation code: {:#X}", operation_code);

        if let Some(operation) = Operation::get_operation(operation_code) {
            let micro_instructions = operation.get_micro_instructions();
            self.decoded_addressing_mode = micro_instructions.addressing_sequence;
            self.decoded_operation = Some(micro_instructions.operation_sequence);
        } else {
            panic!("Operation not found for opcode: {:#X}", operation_code);
        }

        self.step_program_counter();
    }

    pub fn immediate_read<T: BusLike>(&mut self, bus: &mut T) {
        self.memory_buffer = bus.read(self.program_counter);
        self.step_program_counter();
    }

    pub fn read_adl<T: BusLike>(&mut self, bus: &mut T) {
        self.adl = bus.read(self.program_counter);
        self.step_program_counter();
    }

    pub fn read_adh<T: BusLike>(&mut self, bus: &mut T) {
        self.adh = bus.read(self.program_counter);
        self.step_program_counter();
    }

    pub fn read_zero_page<T: BusLike>(&mut self, bus: &mut T) {
        println!("Reading zero page address: {:#X}", self.adl);
        self.memory_buffer = bus.read(self.adl as u16);
    }

    pub fn read_absolute<T: BusLike>(&mut self, bus: &mut T) {
        let address = (self.adh as u16) << 8 | self.adl as u16;
        self.memory_buffer = bus.read(address as u16);
    }

    pub fn read_bal<T: BusLike>(&mut self, bus: &mut T) {
        self.bal = bus.read(self.program_counter as u16);
        self.step_program_counter();
    }

    pub fn read_bah<T: BusLike>(&mut self, bus: &mut T) {
        self.bah = bus.read(self.program_counter as u16);
        self.step_program_counter();
    }

    pub fn read_adl_indirect_bal<T: BusLike>(&mut self, bus: &mut T) {
        let address = (self.bal + self.x) as usize;
        self.adl = bus.read(address as u16);
    }

    pub fn read_adh_indirect_bal<T: BusLike>(&mut self, bus: &mut T) {
        let address = (self.bal + self.x + 1) as usize;
        self.adh = bus.read(address as u16);
    }

    pub fn write_zero_page<T: BusLike>(&mut self, bus: &mut T) {
        bus.write(self.adl as u16, self.memory_buffer);
    }

    pub fn write_absolute<T: BusLike>(&mut self, bus: &mut T) {
        let address = (self.adh as u16) << 8 | self.adl as u16;
        bus.write(address as u16, self.memory_buffer);
    }

    pub fn read_zero_page_bal_x<T: BusLike>(&mut self, bus: &mut T) {
        // TODO: Be careful with overflow, check if it's correct

        let address = (self.bal + self.x) as usize;
        self.memory_buffer = bus.read(address as u16);
    }

    pub fn read_zero_page_bal_y<T: BusLike>(&mut self, bus: &mut T) {
        let address = (self.bal + self.y) as usize;
        self.memory_buffer = bus.read(address as u16);
    }

    pub fn write_zero_page_bal_x<T: BusLike>(&mut self, bus: &mut T) {
        let address = (self.bal + self.x) as usize;
        bus.write(address as u16, self.memory_buffer);
    }

    pub fn read_adl_adh_absolute_index_register<T: BusLike>(
        &mut self,
        bus: &mut T,
        index_register: u8,
    ) {
        let bal_address = self.bal as usize;
        let bah_address = self.bah as usize;
        let address = ((bah_address << 8) | bal_address) + (index_register as usize);
        self.adh = ((address & 0xFF00) >> 8) as u8;
        self.adl = (address & 0x00FF) as u8;

        self.memory_buffer = bus.read(address as u16);
    }

    pub fn read_adl_adh_absolute_x<T: BusLike>(&mut self, bus: &mut T) {
        self.read_adl_adh_absolute_index_register(bus, self.x);
    }

    pub fn read_adl_adh_absolute_y<T: BusLike>(&mut self, bus: &mut T) {
        self.read_adl_adh_absolute_index_register(bus, self.y);
    }

    pub fn read_ial<T: BusLike>(&mut self, bus: &mut T) {
        self.ial = bus.read(self.program_counter as u16);
        self.step_program_counter();
    }

    pub fn read_bal_indirect_ial<T: BusLike>(&mut self, bus: &mut T) {
        self.bal = bus.read(self.ial as u16);
    }

    pub fn read_bah_indirect_ial<T: BusLike>(&mut self, bus: &mut T) {
        self.bah = bus.read(self.ial as u16 + 1);
    }

    pub fn shift_left_accumulator(&mut self) {
        let is_carry = self.a & 0x80 != 0;
        self.a <<= 1;
        let is_negative = self.a & 0x80 != 0;

        self.set_flag_value(CPUFlag::CarryBit, is_carry);
        self.set_flag_value(CPUFlag::Zero, self.a == 0);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    pub fn shift_left_memory_buffer(&mut self) {
        let is_carry = self.memory_buffer & 0x80 != 0;
        self.memory_buffer <<= 1;
        let is_negative = self.memory_buffer & 0x80 != 0;

        self.set_flag_value(CPUFlag::CarryBit, is_carry);
        self.set_flag_value(CPUFlag::Zero, self.memory_buffer == 0);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    pub fn increment_memory_buffer(&mut self) {
        self.memory_buffer = self.memory_buffer.wrapping_add(1u8);
        let is_zero = self.memory_buffer == 0;
        let is_negative = self.memory_buffer & 0x80 != 0;

        self.set_flag_value(CPUFlag::Zero, is_zero);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    pub fn increment_x(&mut self) {
        self.x = self.x.wrapping_add(1u8);
        let is_zero = self.x == 0;
        let is_negative = self.x & 0x80 != 0;

        self.set_flag_value(CPUFlag::Zero, is_zero);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    pub fn increment_y(&mut self) {
        self.y = self.y.wrapping_add(1u8);
        let is_zero = self.y == 0;
        let is_negative = self.x & 0x80 != 0;

        self.set_flag_value(CPUFlag::Zero, is_zero);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    pub fn dec_memory_buffer(&mut self) {
        self.memory_buffer = self.memory_buffer.wrapping_sub(1u8);
        let is_zero = self.memory_buffer == 0;
        let is_negative = self.memory_buffer & 0x80 != 0;

        self.set_flag_value(CPUFlag::Zero, is_zero);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    pub fn dec_x(&mut self) {
        self.x = self.x.wrapping_sub(1u8);
        let is_zero = self.x == 0;
        let is_negative = self.x & 0x80 != 0;

        self.set_flag_value(CPUFlag::Zero, is_zero);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    pub fn dec_y(&mut self) {
        self.y = self.y.wrapping_sub(1u8);
        let is_zero = self.y == 0;
        let is_negative = self.y & 0x80 != 0;

        self.set_flag_value(CPUFlag::Zero, is_zero);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    pub fn load_accumulator(&mut self) {
        self.a = self.memory_buffer;
        let is_zero = self.a == 0;
        let is_negative = self.a & 0x80 != 0;

        self.set_flag_value(CPUFlag::Zero, is_zero);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    pub fn load_x(&mut self) {
        self.x = self.memory_buffer;
        let is_zero = self.x == 0;
        let is_negative = self.x & 0x80 != 0;

        self.set_flag_value(CPUFlag::Zero, is_zero);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    pub fn load_y(&mut self) {
        self.y = self.memory_buffer;
        let is_zero = self.y == 0;
        let is_negative = self.y & 0x80 != 0;

        self.set_flag_value(CPUFlag::Zero, is_zero);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    pub fn and(&mut self) {
        self.a = self.a & self.memory_buffer;
        let is_zero = self.a == 0;
        let is_negative = self.a & 0x80 != 0;

        self.set_flag_value(CPUFlag::Zero, is_zero);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }
}
