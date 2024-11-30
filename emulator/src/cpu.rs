use crate::bus::{Bus, BusLike};

pub struct CPU<T: BusLike> {
    bus: T,
    registers: Registers,
    state: CPUState,
    fetching_operation: MicroInstructionSequence,
    current_micro_instruction: Option<MicroInstruction>,
}

pub struct Registers {
    x: u8,
    y: u8,
    a: u8,
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
    memory_buffer: u8,
}

pub struct MicroInstructionSequence {
    sequence: Vec<MicroInstruction>,
    idx: usize,
}

#[derive(Clone, PartialEq, Debug)]
enum CPUFlag {
    CarryBit,
    Zero,
    InterruptDisable,
    DecimalMode,
    Break,
    Unused,
    Overflow,
    Negative,
}

#[derive(Clone, PartialEq, Debug)]
enum MicroInstruction {
    Empty,
    ReadOperationCode,
    DecodeOperation,

    ImmediateRead,
    ReadAdl,
    ReadAdh,
    ReadZeroPage,
    ReadAbsolute,
    ReadBal,
    ReadBah,
    ReadAdlIndirectBal,
    ReadAdhIndirectBal,
    ReadZeroPageBalX,
    ReadAdlAdhAbsoluteX,
    ReadAdlAdhAbsoluteY,
    ReadIal,
    ReadBalIndirectIal,
    ReadBahIndirectIal,

    WriteZeroPage,
    WriteAbsolute,
    WriteZeroPageBalX,

    ShiftLeftAccumulator,
    ShiftLeftMemoryBuffer,
}

#[derive(PartialEq, Debug)]
enum CPUState {
    Fetching,
    Execution,
}

#[derive(PartialEq, Debug)]
enum Operation {
    AslA,
    AslZeroPage,
    AslZeroPageX,
    AslAbsolute,
}

struct OperationMicroInstructions {
    pub addressing_sequence: Option<MicroInstructionSequence>,
    pub operation_sequence: MicroInstructionSequence,
}

impl Operation {
    fn get_micro_instructions(&self) -> OperationMicroInstructions {
        let zero_page_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadAdl,
            MicroInstruction::ReadZeroPage,
        ]);
        let zero_page_x_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadBal,
            MicroInstruction::Empty, // Because we can add it in the next step easily
            MicroInstruction::ReadZeroPageBalX,
        ]);
        let absolute_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadAdl,
            MicroInstruction::ReadAdh,
            MicroInstruction::ReadAbsolute,
        ]);
        let indirect_x_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadBal,
            MicroInstruction::Empty, // Because we can add it in the next step easily
            MicroInstruction::ReadAdlIndirectBal,
            MicroInstruction::ReadAdhIndirectBal,
            MicroInstruction::ReadAbsolute,
        ]);
        let absolute_x_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadBal,
            MicroInstruction::ReadBah,
            MicroInstruction::ReadAdlAdhAbsoluteX,
            // TODO: Check if this is correct (T4 is optional if page boundary is not crossed)
        ]);
        let absolute_y_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadBal,
            MicroInstruction::ReadBah,
            MicroInstruction::ReadAdlAdhAbsoluteY,
        ]);
        let indirect_y_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadIal,
            MicroInstruction::ReadBalIndirectIal,
            MicroInstruction::ReadBahIndirectIal,
            MicroInstruction::ReadAdlAdhAbsoluteY,
            // TODO: Same as absolute_x_addressing
        ]);

        match self {
            Self::AslA => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::ShiftLeftAccumulator,
                ]),
            },
            Self::AslZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::ShiftLeftMemoryBuffer,
                    MicroInstruction::WriteZeroPage,
                ]),
            },
            Self::AslZeroPageX => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::ShiftLeftMemoryBuffer,
                    MicroInstruction::WriteZeroPageBalX,
                ]),
            },
            Self::AslAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::ShiftLeftMemoryBuffer,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
        }
    }

    fn get_opcode(&self) -> u8 {
        match self {
            Self::AslA => 0x0A,
            Self::AslZeroPage => 0x06,
            Self::AslZeroPageX => 0x16,
            Self::AslAbsolute => 0x0E,
        }
    }

    fn get_operation(opcode: u8) -> Option<Self> {
        match opcode {
            0x0A => Some(Self::AslA),
            0x06 => Some(Self::AslZeroPage),
            0x16 => Some(Self::AslZeroPageX),
            0x0E => Some(Self::AslAbsolute),
            _ => None,
        }
    }
}

impl Registers {
    fn new() -> Self {
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

    fn get_operation(&mut self) -> &mut Option<MicroInstructionSequence> {
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

    fn is_operation_completed(&self) -> bool {
        match &self.decoded_operation {
            Some(operation) => operation.is_completed(),
            None => false,
        }
    }

    fn set_flag(&mut self, flag: CPUFlag) {
        self.status |= flag.value();
    }

    fn clear_flag(&mut self, flag: CPUFlag) {
        self.status &= !flag.value();
    }

    fn set_flag_value(&mut self, flag: CPUFlag, value: bool) {
        if value {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }

    fn is_flag_set(&self, flag: CPUFlag) -> bool {
        self.status & flag.value() != 0
    }

    fn reset_flags(&mut self) {
        self.status = 0x00;
    }

    fn step_program_counter(&mut self) {
        self.program_counter += 1;
    }

    fn read_operation_code<T: BusLike>(&mut self, bus: &mut T) {
        self.operation = bus.read(self.program_counter as u16);
    }

    fn decode_operation<T: BusLike>(&mut self, bus: &T) {
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

    fn immediate_read<T: BusLike>(&mut self, bus: &mut T) {
        self.memory_buffer = bus.read(self.program_counter);
        self.step_program_counter();
    }

    fn read_adl<T: BusLike>(&mut self, bus: &mut T) {
        self.adl = bus.read(self.program_counter);
        self.step_program_counter();
    }

    fn read_adh<T: BusLike>(&mut self, bus: &mut T) {
        self.adh = bus.read(self.program_counter);
        self.step_program_counter();
    }

    fn read_zero_page<T: BusLike>(&mut self, bus: &mut T) {
        println!("Reading zero page address: {:#X}", self.adl);
        self.memory_buffer = bus.read(self.adl as u16);
    }

    fn read_absolute<T: BusLike>(&mut self, bus: &mut T) {
        let address = (self.adh as u16) << 8 | self.adl as u16;
        self.memory_buffer = bus.read(address as u16);
    }

    fn read_bal<T: BusLike>(&mut self, bus: &mut T) {
        self.bal = bus.read(self.program_counter as u16);
        self.step_program_counter();
    }

    fn read_bah<T: BusLike>(&mut self, bus: &mut T) {
        self.bah = bus.read(self.program_counter as u16);
        self.step_program_counter();
    }

    fn read_adl_indirect_bal<T: BusLike>(&mut self, bus: &mut T) {
        let address = (self.bal + self.x) as usize;
        self.adl = bus.read(address as u16);
    }

    fn read_adh_indirect_bal<T: BusLike>(&mut self, bus: &mut T) {
        let address = (self.bal + self.x + 1) as usize;
        self.adh = bus.read(address as u16);
    }

    fn write_zero_page<T: BusLike>(&mut self, bus: &mut T) {
        bus.write(self.adl as u16, self.memory_buffer);
    }

    fn write_absolute<T: BusLike>(&mut self, bus: &mut T) {
        let address = (self.adh as u16) << 8 | self.adl as u16;
        bus.write(address as u16, self.memory_buffer);
    }

    fn read_zero_page_bal_x<T: BusLike>(&mut self, bus: &mut T) {
        // TODO: Be careful with overflow, check if it's correct

        let address = (self.bal + self.x) as usize;
        self.memory_buffer = bus.read(address as u16);
    }

    fn write_zero_page_bal_x<T: BusLike>(&mut self, bus: &mut T) {
        let address = (self.bal + self.x) as usize;
        bus.write(address as u16, self.memory_buffer);
    }

    fn read_adl_adh_absolute_index_register<T: BusLike>(
        &mut self,
        bus: &mut T,
        index_register: u8,
    ) {
        let bal_address = self.bal as usize;
        let bah_address = self.bah as usize;
        let address = ((bah_address << 8) | bal_address) + (index_register as usize);
        self.memory_buffer = bus.read(address as u16);
    }

    fn read_adl_adh_absolute_x<T: BusLike>(&mut self, bus: &mut T) {
        self.read_adl_adh_absolute_index_register(bus, self.x);
    }

    fn read_adl_adh_absolute_y<T: BusLike>(&mut self, bus: &mut T) {
        self.read_adl_adh_absolute_index_register(bus, self.y);
    }

    fn read_ial<T: BusLike>(&mut self, bus: &mut T) {
        self.ial = bus.read(self.program_counter as u16);
        self.step_program_counter();
    }

    fn read_bal_indirect_ial<T: BusLike>(&mut self, bus: &mut T) {
        self.bal = bus.read(self.ial as u16);
    }

    fn read_bah_indirect_ial<T: BusLike>(&mut self, bus: &mut T) {
        self.bah = bus.read(self.ial as u16 + 1);
    }

    fn shift_left_accumulator(&mut self) {
        let is_carry = self.a & 0x80 != 0;
        self.a <<= 1;
        let is_negative = self.a & 0x80 != 0;

        self.set_flag_value(CPUFlag::CarryBit, is_carry);
        self.set_flag_value(CPUFlag::Zero, self.a == 0);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }

    fn shift_left_memory_buffer(&mut self) {
        let is_carry = self.memory_buffer & 0x80 != 0;
        self.memory_buffer <<= 1;
        let is_negative = self.memory_buffer & 0x80 != 0;

        self.set_flag_value(CPUFlag::CarryBit, is_carry);
        self.set_flag_value(CPUFlag::Zero, self.memory_buffer == 0);
        self.set_flag_value(CPUFlag::Negative, is_negative);
    }
}

impl<T: BusLike> CPU<T> {
    fn new(bus: T) -> Self {
        let registers = Registers::new();
        let state = CPUState::Fetching;
        let fetching_operations = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadOperationCode,
            MicroInstruction::DecodeOperation,
        ]);

        Self {
            bus,
            registers,
            state,
            fetching_operation: fetching_operations,
            current_micro_instruction: None,
        }
    }

    fn step(&mut self) {
        match self.state {
            CPUState::Fetching => {
                self.fetch_step();
            }
            CPUState::Execution => {
                self.execute_step();
            }
        }

        let current_micro_instruction = self.current_micro_instruction.clone();
        if let Some(micro_instruction) = current_micro_instruction {
            self.execute_micro_instruction(&micro_instruction);
        }
    }

    fn fetch_step(&mut self) {
        let micro_instruction = self.fetching_operation.get_micro_instruction().clone();
        self.current_micro_instruction = Some(micro_instruction);
        self.fetching_operation.next();

        if self.fetching_operation.is_completed() {
            self.fetching_operation.reset();
            self.state = CPUState::Execution;
        }
    }

    fn execute_step(&mut self) {
        match self.registers.get_operation() {
            Some(ref mut operation) => {
                let micro_instruction = operation.get_micro_instruction().clone();
                self.current_micro_instruction = Some(micro_instruction);
                operation.next();

                if self.registers.is_operation_completed() {
                    self.state = CPUState::Fetching;
                }
            }
            None => {
                panic!("No instruction to execute.")
            }
        }
    }

    fn execute_micro_instruction(&mut self, micro_instruction: &MicroInstruction) {
        match micro_instruction {
            MicroInstruction::Empty => (),

            MicroInstruction::ReadOperationCode => {
                self.registers.read_operation_code(&mut self.bus)
            }
            MicroInstruction::DecodeOperation => self.registers.decode_operation(&mut self.bus),
            MicroInstruction::ImmediateRead => self.registers.immediate_read(&mut self.bus),
            MicroInstruction::ReadAdh => self.registers.read_adh(&mut self.bus),
            MicroInstruction::ReadAdl => self.registers.read_adl(&mut self.bus),
            MicroInstruction::ReadZeroPage => self.registers.read_zero_page(&mut self.bus),
            MicroInstruction::ReadAbsolute => self.registers.read_absolute(&mut self.bus),
            MicroInstruction::ReadBal => self.registers.read_bal(&mut self.bus),
            MicroInstruction::ReadBah => self.registers.read_bah(&mut self.bus),
            MicroInstruction::ReadAdlIndirectBal => {
                self.registers.read_adl_indirect_bal(&mut self.bus)
            }
            MicroInstruction::ReadAdhIndirectBal => {
                self.registers.read_adh_indirect_bal(&mut self.bus)
            }
            MicroInstruction::ReadZeroPageBalX => {
                self.registers.read_zero_page_bal_x(&mut self.bus)
            }
            MicroInstruction::ReadAdlAdhAbsoluteX => {
                self.registers.read_adl_adh_absolute_x(&mut self.bus)
            }
            MicroInstruction::ReadAdlAdhAbsoluteY => {
                self.registers.read_adl_adh_absolute_y(&mut self.bus)
            }
            MicroInstruction::ReadIal => self.registers.read_ial(&mut self.bus),
            MicroInstruction::ReadBalIndirectIal => {
                self.registers.read_bal_indirect_ial(&mut self.bus)
            }
            MicroInstruction::ReadBahIndirectIal => {
                self.registers.read_bah_indirect_ial(&mut self.bus)
            }

            MicroInstruction::WriteZeroPage => self.registers.write_zero_page(&mut self.bus),
            MicroInstruction::WriteAbsolute => self.registers.write_absolute(&mut self.bus),
            MicroInstruction::WriteZeroPageBalX => {
                self.registers.write_zero_page_bal_x(&mut self.bus)
            }

            MicroInstruction::ShiftLeftAccumulator => self.registers.shift_left_accumulator(),
            MicroInstruction::ShiftLeftMemoryBuffer => self.registers.shift_left_memory_buffer(),
        }
    }
}

impl MicroInstructionSequence {
    fn new(sequence: Vec<MicroInstruction>) -> Self {
        Self { sequence, idx: 0 }
    }

    fn get_micro_instruction(&self) -> &MicroInstruction {
        &self.sequence[self.idx]
    }

    fn next(&mut self) {
        self.idx += 1;
    }

    fn is_completed(&self) -> bool {
        self.idx >= self.sequence.len()
    }

    fn reset(&mut self) {
        self.idx = 0;
    }
}
impl CPUFlag {
    fn value(&self) -> u8 {
        match *self {
            Self::CarryBit => 1 << 0,
            Self::Zero => 1 << 1,
            Self::InterruptDisable => 1 << 2,
            Self::DecimalMode => 1 << 3,
            Self::Break => 1 << 4,
            Self::Unused => 1 << 5,
            Self::Overflow => 1 << 6,
            Self::Negative => 1 << 7,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bus;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    struct TestBus {
        memory: Vec<usize>,
    }

    impl TestBus {
        pub fn new() -> Self {
            Self {
                memory: vec![0; bus::ADDRESS_SPACE],
            }
        }
    }

    impl BusLike for TestBus {
        fn read(&mut self, address: u16) -> u8 {
            self.memory[address as usize] as u8
        }

        fn write(&mut self, address: u16, data: u8) {
            println!("Writing {:#X} to address {:#X}", data, address);
            self.memory[address as usize] = data as usize;
        }
    }

    #[test]
    fn test_cpu_new() {
        let bus = TestBus::new();
        let cpu = CPU::new(bus);

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, None);
    }

    #[test]
    fn test_cpu_fetch_step() {
        let bus = TestBus::new();
        let mut cpu = CPU::new(bus);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadOperationCode)
        );
    }

    #[test]
    fn test_cpu_asl_a() {
        const OPCODE: u8 = 0x0A;
        let mut bus = TestBus::new();
        bus.write(0, OPCODE);
        let mut cpu = CPU::new(bus);

        cpu.step();
        cpu.step();

        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.state, CPUState::Execution);

        cpu.step();

        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ShiftLeftAccumulator)
        );
    }

    #[test]
    fn test_cpu_asl_a_not_empty() {
        const OPCODE: u8 = 0x0A;
        let mut bus = TestBus::new();
        bus.write(0, OPCODE);
        let mut cpu = CPU::new(bus);

        cpu.registers.a = 0b10000000;

        cpu.step();
        cpu.step();

        assert_eq!(cpu.registers.a, 0b10000000);
        assert_eq!(cpu.state, CPUState::Execution);

        cpu.step();

        assert_eq!(cpu.registers.a, 0b00000000);
        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ShiftLeftAccumulator)
        );
    }

    #[test]
    fn test_cpu_asl_zero_page() {
        const OPCODE: u8 = 0x06;
        const ADDRESS: u8 = 0x10;
        const VALUE: u8 = 0b10;
        const EXPECTED_VALUE: u8 = 0b100;

        let mut bus = TestBus::new();
        bus.write(0, OPCODE);
        bus.write(1, ADDRESS);
        bus.write(ADDRESS as u16, VALUE);

        let mut cpu = CPU::new(bus);

        cpu.step();
        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::DecodeOperation)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadAdl)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadZeroPage)
        );

        cpu.step();
        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteZeroPage)
        );

        let read_value = cpu.bus.read(ADDRESS as u16);

        assert_eq!(read_value, EXPECTED_VALUE);
    }
}
