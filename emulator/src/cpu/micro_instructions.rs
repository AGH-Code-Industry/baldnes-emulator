#[derive(Clone, PartialEq, Debug)]
pub enum MicroInstruction {
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
    ReadZeroPageBalY,
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

    IncrementMemoryBuffer,
    IncrementX,
    IncrementY,
    DecrementMemoryBuffer,
    DecrementX,
    DecrementY,

    LoadAccumulator,
    LoadX,
    LoadY,

    And,
}

pub struct MicroInstructionSequence {
    sequence: Vec<MicroInstruction>,
    idx: usize,
}

impl MicroInstructionSequence {
    pub fn new(sequence: Vec<MicroInstruction>) -> Self {
        Self { sequence, idx: 0 }
    }

    pub fn get_micro_instruction(&self) -> &MicroInstruction {
        &self.sequence[self.idx]
    }

    pub fn next(&mut self) {
        self.idx += 1;
    }

    pub fn is_completed(&self) -> bool {
        self.idx >= self.sequence.len()
    }

    pub fn reset(&mut self) {
        self.idx = 0;
    }
}
