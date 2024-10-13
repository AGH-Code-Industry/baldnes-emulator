use crate::bus::Bus;

pub struct CPU {
    bus: Bus,
    registers: Registers,
    state: CPUState,
    fetching_operation: MicroInstructionSequence,
    current_micro_instruction: Option<MicroInstruction>,
}

pub struct Registers {
    pub x: u8,
    pub y: u8,
    pub a: u8,
    pub program_counter: u16,
    pub stack_ptr: u8,
    pub status: u8,
    pub operation: u8,
    pub adl: u8,
    pub adh: u8,
    pub bal: u8,
    pub bah: u8,
    pub decoded_addressing_mode: Option<MicroInstructionSequence>,
    pub decoded_operation: Option<MicroInstructionSequence>,
    pub memory_buffer: u8,
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
}

enum CPUState {
    Fetching,
    Execution,
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
        if let None = self.decoded_operation {
            return &mut self.decoded_operation;
        }
        if let Some(ref mut decoded_addressing_mode) = self.decoded_addressing_mode {
            if decoded_addressing_mode.is_completed() {
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

        // TODO: Implement instruction decoding
    }

    fn immediate_read(&mut self, bus: &Bus) {
        self.memory_buffer = bus.read(self.program_counter as usize);
    }

    fn read_adl(&mut self, bus: &Bus) {
        self.adl = bus.read(self.program_counter as usize);
    }

    fn read_adh(&mut self, bus: &Bus) {
        self.adh = bus.read(self.program_counter as usize);
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
    }

    fn read_bah(&mut self, bus: &Bus) {
        self.bah = bus.read(self.program_counter as usize);
    }

    fn read_adl_indirect_bal(&mut self, bus: &Bus) {
        let address = (self.bal + self.x) as usize;
        self.adl = bus.read(address);
    }

    fn read_adh_indirect_bal(&mut self, bus: &Bus) {
        let address = (self.bal + self.x + 1) as usize;
        self.adh = bus.read(address);
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
