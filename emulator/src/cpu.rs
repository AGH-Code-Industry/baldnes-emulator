use crate::bus::Bus;

pub struct CPU {
    bus: Bus,
    registers: Registers,
    state: CPUState,
    fetching_instruction: MicroInstructionSequence,
    decoded_instruction: Option<MicroInstructionSequence>,
    current_micro_instruction: Option<MicroInstruction>,
}

pub struct Registers {
    pub x: u8,
    pub y: u8,
    pub a: u8,
    pub program_counter: u16,
    pub stack_ptr: u8,
    pub status: u8,
    pub instruction: u8,
}

struct MicroInstructionSequence {
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
    ReadInstructionCode,
    DecodeInstruction,
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
            instruction: 0x00,
        }
    }

    fn read_instruction_code(&mut self, bus: &Bus) {
        self.instruction = bus.read(self.program_counter as usize);
    }

    fn decode_instruction(&mut self, bus: &Bus) {
        let instruction_code = self.instruction;
        println!("Instruction code: {:#X}", instruction_code);

        // TODO: Implement instruction decoding
    }
}

impl CPU {
    fn new(bus: Bus) -> Self {
        let registers = Registers::new();
        let state = CPUState::Fetching;
        let fetching_instruction = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadInstructionCode,
            MicroInstruction::DecodeInstruction,
        ]);

        Self {
            bus,
            registers,
            state,
            fetching_instruction,
            decoded_instruction: None,
            current_micro_instruction: None,
        }
    }
    fn micro_cycle(&mut self) {
        match self.state {
            CPUState::Fetching => {
                self.fetch_cycle();
            }
            CPUState::Execution => {
                self.execute_cycle();
            }
        }

        let current_micro_instruction = self.current_micro_instruction.clone();
        if let Some(micro_instruction) = current_micro_instruction {
            self.execute_instruction(&micro_instruction);
        }
    }

    fn fetch_cycle(&mut self) {
        let micro_instruction = self.fetching_instruction.get_micro_instruction().clone();
        self.current_micro_instruction = Some(micro_instruction);
        self.fetching_instruction.next();

        if self.fetching_instruction.is_completed() {
            self.fetching_instruction.reset();
            self.state = CPUState::Execution;
        }
    }

    fn execute_cycle(&mut self) {
        match self.decoded_instruction {
            Some(ref mut instruction) => {
                let micro_instruction = instruction.get_micro_instruction().clone();
                self.current_micro_instruction = Some(micro_instruction);
                instruction.next();

                if instruction.is_completed() {
                    instruction.reset();
                    self.state = CPUState::Fetching;
                }
            }
            None => {
                panic!("No instruction to execute.")
            }
        }
    }

    fn execute_instruction(&mut self, micro_instruction: &MicroInstruction) {
        match micro_instruction {
            MicroInstruction::ReadInstructionCode => {
                self.registers.read_instruction_code(&self.bus)
            }
            MicroInstruction::DecodeInstruction => self.registers.decode_instruction(&self.bus),
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
