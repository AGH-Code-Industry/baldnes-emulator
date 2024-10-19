use crate::bus::Bus;

pub struct CPU {
    bus: Bus,
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
    decoded_addressing_mode: Option<MicroInstructionSequence>,
    decoded_operation: Option<MicroInstructionSequence>,
    memory_buffer: u8,
}

pub struct MicroInstructionSequence {
    sequence: Vec<MicroInstruction>,
    idx: usize,
}

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

#[derive(Clone)]
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

    WriteZeroPage,
    WriteAbsolute,
    WriteZeroPageBalX,

    ShiftLeftAccumulator,
    ShiftLeftMemoryBuffer,
}

enum CPUState {
    Fetching,
    Execution,
}

enum Operation {
    AslA,
    AslZeroPage,
    AslZeroPageX,
    AslAbsolute,
}

struct OperationMicroInstructions {
    pub addressing_sequence: Option<MicroInstructionSequence>,
    pub operation_sequence: MicroInstructionSequence
}

impl Operation {
    fn get_micro_instructions(&self) -> OperationMicroInstructions {
        let zero_page_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadAdl,
            MicroInstruction::ReadZeroPage
        ]);
        let zero_page_x_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadBal,
            MicroInstruction::Empty, // Because we can add it in the next step easily
            MicroInstruction::ReadZeroPageBalX
        ]);
        let absolute_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadAdl,
            MicroInstruction::ReadAdh,
            MicroInstruction::ReadAbsolute
        ]);

        match self {
            Self::AslA => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::ShiftLeftAccumulator
                ])
            },
            Self::AslZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::ShiftLeftMemoryBuffer,
                    MicroInstruction::WriteZeroPage
                ])
            },
            Self::AslZeroPageX => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::ShiftLeftMemoryBuffer,
                    MicroInstruction::WriteZeroPageBalX
                ])
            },
            Self::AslAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::ShiftLeftMemoryBuffer,
                    MicroInstruction::WriteAbsolute
                ])
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
            decoded_addressing_mode: None,
            decoded_operation: None,
            memory_buffer: 0x00,
        }
    }

    fn get_operation(&mut self) -> &mut Option<MicroInstructionSequence> {
        match self.decoded_operation {
            Some(ref mut decoded_addressing_mode) => {
                if decoded_addressing_mode.is_completed() {
                    return &mut self.decoded_operation;
                }
            }
            None => {
                return &mut self.decoded_operation;
            }
        }
        
        &mut self.decoded_addressing_mode
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

    fn read_operation_code(&mut self, bus: &Bus) {
        self.operation = bus.read(self.program_counter as usize);
    }

    fn decode_operation(&mut self, bus: &Bus) {
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

    fn immediate_read(&mut self, bus: &Bus) {
        self.memory_buffer = bus.read(self.program_counter as usize);
        self.step_program_counter();
    }

    fn read_adl(&mut self, bus: &Bus) {
        self.adl = bus.read(self.program_counter as usize);
        self.step_program_counter();
    }

    fn read_adh(&mut self, bus: &Bus) {
        self.adh = bus.read(self.program_counter as usize);
        self.step_program_counter();
    }

    fn read_zero_page(&mut self, bus: &Bus) {
        self.memory_buffer = bus.read(self.adl as usize);
    }

    fn read_absolute(&mut self, bus: &Bus) {
        let address = (self.adh as u16) << 8 | self.adl as u16;
        self.memory_buffer = bus.read(address as usize);
    }

    fn read_bal(&mut self, bus: &Bus) {
        self.bal = bus.read(self.program_counter as usize);
        self.step_program_counter();
    }

    fn read_bah(&mut self, bus: &Bus) {
        self.bah = bus.read(self.program_counter as usize);
        self.step_program_counter();
    }

    fn read_adl_indirect_bal(&mut self, bus: &Bus) {
        let address = (self.bal + self.x) as usize;
        self.adl = bus.read(address);
    }

    fn read_adh_indirect_bal(&mut self, bus: &Bus) {
        let address = (self.bal + self.x + 1) as usize;
        self.adh = bus.read(address);
    }

    fn write_zero_page(&mut self, bus: &mut Bus) {
        bus.write(self.adl as usize, self.memory_buffer);
    }

    fn write_absolute(&mut self, bus: &mut Bus) {
        let address = (self.adh as u16) << 8 | self.adl as u16;
        bus.write(address as usize, self.memory_buffer);
    }

    fn read_zero_page_bal_x(&mut self, bus: &Bus) {
        // TODO: Be careful with overflow, check if it's correct

        let address = (self.bal + self.x) as usize;
        self.memory_buffer = bus.read(address);
    }

    fn write_zero_page_bal_x(&mut self, bus: &mut Bus) {
        let address = (self.bal + self.x) as usize;
        bus.write(address, self.memory_buffer);
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

impl CPU {
    fn new(bus: Bus) -> Self {
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

                if operation.is_completed() {
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

            MicroInstruction::ReadOperationCode => self.registers.read_operation_code(&self.bus),
            MicroInstruction::DecodeOperation => self.registers.decode_operation(&self.bus),
            MicroInstruction::ImmediateRead => self.registers.immediate_read(&self.bus),
            MicroInstruction::ReadAdh => self.registers.read_adh(&self.bus),
            MicroInstruction::ReadAdl => self.registers.read_adl(&self.bus),
            MicroInstruction::ReadZeroPage => self.registers.read_zero_page(&self.bus),
            MicroInstruction::ReadAbsolute => self.registers.read_absolute(&self.bus),
            MicroInstruction::ReadBal => self.registers.read_bal(&self.bus),
            MicroInstruction::ReadBah => self.registers.read_bah(&self.bus),
            MicroInstruction::ReadAdlIndirectBal => self.registers.read_adl_indirect_bal(&self.bus),
            MicroInstruction::ReadAdhIndirectBal => self.registers.read_adh_indirect_bal(&self.bus),
            MicroInstruction::ReadZeroPageBalX => self.registers.read_zero_page_bal_x(&self.bus),
            
            MicroInstruction::WriteZeroPage => self.registers.write_zero_page(&mut self.bus),
            MicroInstruction::WriteAbsolute => self.registers.write_absolute(&mut self.bus),
            MicroInstruction::WriteZeroPageBalX => self.registers.write_zero_page_bal_x(&mut self.bus),

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
