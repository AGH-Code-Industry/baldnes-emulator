use crate::cpu::micro_instructions::{MicroInstruction, MicroInstructionSequence};

#[derive(PartialEq, Debug)]
pub enum Operation {
    AslA,
    AslZeroPage,
    AslZeroPageX,
    AslAbsolute,

    IncMemZeroPage,
    IncMemZeroPageX,
    IncMemAbsolute,
    IncMemAbsoluteX,
    IncX,
    IncY,

    DecMemZeroPage,
    DecMemZeroPageX,
    DecMemAbsolute,
    DecMemAbsoluteX,
    DecX,
    DecY,

    LoadAccImm,
    LoadAccZeroPage,
    LoadAccZeroPageX,
    LoadAccAbsolute,
    LoadAccAbsoluteX,
    LoadAccAbsoluteY,
    LoadAccIndirectX,
    LoadAccIndirectY,
    LoadXImm,
    LoadXZeroPage,
    LoadXZeroPageY,
    LoadXAbsolute,
    LoadXAbsoluteY,
    LoadYImm,
    LoadYZeroPage,
    LoadYZeroPageX,
    LoadYAbsolute,
    LoadYAbsoluteX,

    StoreAccZeroPage,
    StoreAccZeroPageX,
    StoreAccAbsolute,
    StoreAccAbsoluteX,
    StoreAccAbsoluteY,
    StoreAccIndirectX,
    StoreAccIndirectY,
    StoreXZeroPage,
    StoreXZeroPageY,
    StoreXAbsolute,
    StoreYZeroPage,
    StoreYZeroPageX,
    StoreYAbsolute,

    TransferAccToX,
    TransferAccToY,
    TransferStackptrToX,
    TransferXToAcc,
    TransferXToStackptr,
    TransferYToAcc,

    AndImm,
    AndZeroPage,
    AndZeroPageX,
    AndAbsolute,
    AndAbsoluteX,
    AndAbsoluteY,
    AndIndirectX,
    AndIndirectY,

    XorImm,
    XorZeroPage,
    XorZeroPageX,
    XorAbsolute,
    XorAbsoluteX,
    XorAbsoluteY,
    XorIndirectX,
    XorIndirectY,
}

pub struct OperationMicroInstructions {
    pub addressing_sequence: Option<MicroInstructionSequence>,
    pub operation_sequence: MicroInstructionSequence,
}

impl Operation {
    pub fn get_micro_instructions(&self) -> OperationMicroInstructions {
        let zero_page_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadAdl,
            MicroInstruction::ReadZeroPage,
        ]);
        let zero_page_x_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadBal,
            MicroInstruction::Empty, // Because we can add it in the next step easily
            MicroInstruction::ReadZeroPageBalX,
        ]);
        let zero_page_y_addressing = MicroInstructionSequence::new(vec![
            MicroInstruction::ReadBal,
            MicroInstruction::Empty,
            MicroInstruction::ReadZeroPageBalY,
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
        let immediate_addressing =
            MicroInstructionSequence::new(vec![MicroInstruction::ImmediateRead]);

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
            Self::IncMemZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::IncrementMemoryBuffer,
                    MicroInstruction::WriteZeroPage,
                ]),
            },
            Self::IncMemZeroPageX => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::IncrementMemoryBuffer,
                    MicroInstruction::WriteZeroPageBalX,
                ]),
            },
            Self::IncMemAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::IncrementMemoryBuffer,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
            Self::IncMemAbsoluteX => OperationMicroInstructions {
                addressing_sequence: Some(absolute_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::IncrementMemoryBuffer,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
            Self::IncX => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::IncrementX,
                ]),
            },
            Self::IncY => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::IncrementY,
                ]),
            },
            Self::DecMemZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::DecrementMemoryBuffer,
                    MicroInstruction::WriteZeroPage,
                ]),
            },
            Self::DecMemZeroPageX => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::DecrementMemoryBuffer,
                    MicroInstruction::WriteZeroPageBalX,
                ]),
            },
            Self::DecMemAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::DecrementMemoryBuffer,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
            Self::DecMemAbsoluteX => OperationMicroInstructions {
                addressing_sequence: Some(absolute_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::DecrementMemoryBuffer,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
            Self::DecX => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::DecrementX,
                ]),
            },
            Self::DecY => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::DecrementY,
                ]),
            },
            Self::LoadAccImm => OperationMicroInstructions {
                addressing_sequence: Some(immediate_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::LoadAccumulator,
                ]),
            },
            Self::LoadAccZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::LoadAccumulator,
                ]),
            },
            Self::LoadAccZeroPageX => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::LoadAccumulator,
                ]),
            },
            Self::LoadAccAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::LoadAccumulator,
                ]),
            },
            Self::LoadAccAbsoluteX => OperationMicroInstructions {
                addressing_sequence: Some(absolute_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::LoadAccumulator,
                ]),
            },
            Self::LoadAccAbsoluteY => OperationMicroInstructions {
                addressing_sequence: Some(absolute_y_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::LoadAccumulator,
                ]),
            },
            Self::LoadAccIndirectX => OperationMicroInstructions {
                addressing_sequence: Some(indirect_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::LoadAccumulator,
                ]),
            },
            Self::LoadAccIndirectY => OperationMicroInstructions {
                addressing_sequence: Some(indirect_y_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::LoadAccumulator,
                ]),
            },
            Self::LoadXImm => OperationMicroInstructions {
                addressing_sequence: Some(immediate_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::LoadX]),
            },
            Self::LoadXZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::LoadX]),
            },
            Self::LoadXZeroPageY => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_y_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::LoadX]),
            },
            Self::LoadXAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::LoadX]),
            },
            Self::LoadXAbsoluteY => OperationMicroInstructions {
                addressing_sequence: Some(absolute_y_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::LoadX]),
            },
            Self::LoadYImm => OperationMicroInstructions {
                addressing_sequence: Some(immediate_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::LoadY]),
            },
            Self::LoadYZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::LoadY]),
            },
            Self::LoadYZeroPageX => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::LoadY]),
            },
            Self::LoadYAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::LoadY]),
            },
            Self::LoadYAbsoluteX => OperationMicroInstructions {
                addressing_sequence: Some(absolute_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::LoadY]),
            },
            Self::StoreAccZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreAccumulator,
                    MicroInstruction::WriteZeroPage,
                ]),
            },
            Self::StoreAccZeroPageX => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreAccumulator,
                    MicroInstruction::WriteZeroPageBalX,
                ]),
            },
            Self::StoreAccAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreAccumulator,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
            Self::StoreAccAbsoluteX => OperationMicroInstructions {
                addressing_sequence: Some(absolute_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreAccumulator,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
            Self::StoreAccAbsoluteY => OperationMicroInstructions {
                addressing_sequence: Some(absolute_y_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreAccumulator,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
            Self::StoreAccIndirectX => OperationMicroInstructions {
                addressing_sequence: Some(indirect_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreAccumulator,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
            Self::StoreAccIndirectY => OperationMicroInstructions {
                addressing_sequence: Some(indirect_y_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreAccumulator,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
            Self::StoreXZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreX,
                    MicroInstruction::WriteZeroPage,
                ]),
            },
            Self::StoreXZeroPageY => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_y_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreX,
                    MicroInstruction::WriteZeroPageBalY,
                ]),
            },
            Self::StoreXAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreX,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
            Self::StoreYZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreY,
                    MicroInstruction::WriteZeroPage,
                ]),
            },
            Self::StoreYZeroPageX => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreY,
                    MicroInstruction::WriteZeroPageBalX,
                ]),
            },
            Self::StoreYAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::StoreY,
                    MicroInstruction::WriteAbsolute,
                ]),
            },
            Self::TransferAccToX => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::TransferAccumulatorToX,
                ]),
            },
            Self::TransferAccToY => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::TransferAccumulatorToY,
                ]),
            },
            Self::TransferStackptrToX => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::TransferStackptrToX,
                ]),
            },
            Self::TransferXToAcc => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::TransferXToAccumulator,
                ]),
            },
            Self::TransferXToStackptr => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::TransferXToStackptr,
                ]),
            },
            Self::TransferYToAcc => OperationMicroInstructions {
                addressing_sequence: None,
                operation_sequence: MicroInstructionSequence::new(vec![
                    MicroInstruction::TransferYToAccumulator,
                ]),
            },
            Self::AndImm => OperationMicroInstructions {
                addressing_sequence: Some(immediate_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::And]),
            },
            Self::AndZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::And]),
            },
            Self::AndZeroPageX => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::And]),
            },
            Self::AndAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::And]),
            },
            Self::AndAbsoluteX => OperationMicroInstructions {
                addressing_sequence: Some(absolute_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::And]),
            },
            Self::AndAbsoluteY => OperationMicroInstructions {
                addressing_sequence: Some(absolute_y_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::And]),
            },
            Self::AndIndirectX => OperationMicroInstructions {
                addressing_sequence: Some(indirect_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::And]),
            },
            Self::AndIndirectY => OperationMicroInstructions {
                addressing_sequence: Some(indirect_y_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::And]),
            },
            Self::XorImm => OperationMicroInstructions {
                addressing_sequence: Some(immediate_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::Xor]),
            },
            Self::XorZeroPage => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::Xor]),
            },
            Self::XorZeroPageX => OperationMicroInstructions {
                addressing_sequence: Some(zero_page_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::Xor]),
            },
            Self::XorAbsolute => OperationMicroInstructions {
                addressing_sequence: Some(absolute_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::Xor]),
            },
            Self::XorAbsoluteX => OperationMicroInstructions {
                addressing_sequence: Some(absolute_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::Xor]),
            },
            Self::XorAbsoluteY => OperationMicroInstructions {
                addressing_sequence: Some(absolute_y_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::Xor]),
            },
            Self::XorIndirectX => OperationMicroInstructions {
                addressing_sequence: Some(indirect_x_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::Xor]),
            },
            Self::XorIndirectY => OperationMicroInstructions {
                addressing_sequence: Some(indirect_y_addressing),
                operation_sequence: MicroInstructionSequence::new(vec![MicroInstruction::Xor]),
            },
        }
    }

    pub fn get_opcode(&self) -> u8 {
        match self {
            Self::AslA => 0x0A,
            Self::AslZeroPage => 0x06,
            Self::AslZeroPageX => 0x16,
            Self::AslAbsolute => 0x0E,
            Self::IncMemZeroPage => 0xE6,
            Self::IncMemZeroPageX => 0xF6,
            Self::IncMemAbsolute => 0xEE,
            Self::IncMemAbsoluteX => 0xFE,
            Self::IncX => 0xE8,
            Self::IncY => 0xC8,
            Self::DecMemZeroPage => 0xC6,
            Self::DecMemZeroPageX => 0xD6,
            Self::DecMemAbsolute => 0xCE,
            Self::DecMemAbsoluteX => 0xDE,
            Self::DecX => 0xCA,
            Self::DecY => 0x88,
            Self::LoadAccImm => 0xA9,
            Self::LoadAccZeroPage => 0xA5,
            Self::LoadAccZeroPageX => 0xB5,
            Self::LoadAccAbsolute => 0xAD,
            Self::LoadAccAbsoluteX => 0xBD,
            Self::LoadAccAbsoluteY => 0xB9,
            Self::LoadAccIndirectX => 0xA1,
            Self::LoadAccIndirectY => 0xB1,
            Self::LoadXImm => 0xA2,
            Self::LoadXZeroPage => 0xA6,
            Self::LoadXZeroPageY => 0xB6,
            Self::LoadXAbsolute => 0xAE,
            Self::LoadXAbsoluteY => 0xBE,
            Self::LoadYImm => 0xA0,
            Self::LoadYZeroPage => 0xA4,
            Self::LoadYZeroPageX => 0xB4,
            Self::LoadYAbsolute => 0xAC,
            Self::LoadYAbsoluteX => 0xBC,
            Self::StoreAccZeroPage => 0x85,
            Self::StoreAccZeroPageX => 0x95,
            Self::StoreAccAbsolute => 0x8D,
            Self::StoreAccAbsoluteX => 0x9D,
            Self::StoreAccAbsoluteY => 0x99,
            Self::StoreAccIndirectX => 0x81,
            Self::StoreAccIndirectY => 0x91,
            Self::StoreXZeroPage => 0x86,
            Self::StoreXZeroPageY => 0x96,
            Self::StoreXAbsolute => 0x8E,
            Self::StoreYZeroPage => 0x84,
            Self::StoreYZeroPageX => 0x94,
            Self::StoreYAbsolute => 0x8C,
            Self::TransferAccToX => 0xAA,
            Self::TransferAccToY => 0xA8,
            Self::TransferStackptrToX => 0xBA,
            Self::TransferXToAcc => 0x8A,
            Self::TransferXToStackptr => 0x9A,
            Self::TransferYToAcc => 0x98,
            Self::XorImm => 0x49,
            Self::XorZeroPage => 0x45,
            Self::XorZeroPageX => 0x55,
            Self::XorAbsolute => 0x4D,
            Self::XorAbsoluteX => 0x5D,
            Self::XorAbsoluteY => 0x59,
            Self::XorIndirectX => 0x41,
            Self::XorIndirectY => 0x51,
            Self::AndImm => 0x29,
            Self::AndZeroPage => 0x25,
            Self::AndZeroPageX => 0x35,
            Self::AndAbsolute => 0x2D,
            Self::AndAbsoluteX => 0x3D,
            Self::AndAbsoluteY => 0x39,
            Self::AndIndirectX => 0x21,
            Self::AndIndirectY => 0x31,
        }
    }

    pub fn get_operation(opcode: u8) -> Option<Self> {
        match opcode {
            0x0A => Some(Self::AslA),
            0x06 => Some(Self::AslZeroPage),
            0x16 => Some(Self::AslZeroPageX),
            0x0E => Some(Self::AslAbsolute),
            0xE6 => Some(Self::IncMemZeroPage),
            0xF6 => Some(Self::IncMemZeroPageX),
            0xEE => Some(Self::IncMemAbsolute),
            0xFE => Some(Self::IncMemAbsoluteX),
            0xE8 => Some(Self::IncX),
            0xC8 => Some(Self::IncY),
            0xC6 => Some(Self::DecMemZeroPage),
            0xD6 => Some(Self::DecMemZeroPageX),
            0xCE => Some(Self::DecMemAbsolute),
            0xDE => Some(Self::DecMemAbsoluteX),
            0xCA => Some(Self::DecX),
            0x88 => Some(Self::DecY),
            0xA9 => Some(Self::LoadAccImm),
            0xA5 => Some(Self::LoadAccZeroPage),
            0xB5 => Some(Self::LoadAccZeroPageX),
            0xAD => Some(Self::LoadAccAbsolute),
            0xBD => Some(Self::LoadAccAbsoluteX),
            0xB9 => Some(Self::LoadAccAbsoluteY),
            0xA1 => Some(Self::LoadAccIndirectX),
            0xB1 => Some(Self::LoadAccIndirectY),
            0xA2 => Some(Self::LoadXImm),
            0xA6 => Some(Self::LoadXZeroPage),
            0xB6 => Some(Self::LoadXZeroPageY),
            0xAE => Some(Self::LoadXAbsolute),
            0xBE => Some(Self::LoadXAbsoluteY),
            0xA0 => Some(Self::LoadYImm),
            0xA4 => Some(Self::LoadYZeroPage),
            0xB4 => Some(Self::LoadYZeroPageX),
            0xAC => Some(Self::LoadYAbsolute),
            0xBC => Some(Self::LoadYAbsoluteX),
            0x85 => Some(Self::StoreAccZeroPage),
            0x95 => Some(Self::StoreAccZeroPageX),
            0x8D => Some(Self::StoreAccAbsolute),
            0x9D => Some(Self::StoreAccAbsoluteX),
            0x99 => Some(Self::StoreAccAbsoluteY),
            0x81 => Some(Self::StoreAccIndirectX),
            0x91 => Some(Self::StoreAccIndirectY),
            0x86 => Some(Self::StoreXZeroPage),
            0x96 => Some(Self::StoreXZeroPageY),
            0x8E => Some(Self::StoreXAbsolute),
            0x84 => Some(Self::StoreYZeroPage),
            0x94 => Some(Self::StoreYZeroPageX),
            0x8C => Some(Self::StoreYAbsolute),
            0xAA => Some(Self::TransferAccToX),
            0xA8 => Some(Self::TransferAccToY),
            0xBA => Some(Self::TransferStackptrToX),
            0x8A => Some(Self::TransferXToAcc),
            0x9A => Some(Self::TransferXToStackptr),
            0x98 => Some(Self::TransferYToAcc),
            0x49 => Some(Self::XorImm),
            0x45 => Some(Self::XorZeroPage),
            0x55 => Some(Self::XorZeroPageX),
            0x4D => Some(Self::XorAbsolute),
            0x5D => Some(Self::XorAbsoluteX),
            0x59 => Some(Self::XorAbsoluteY),
            0x41 => Some(Self::XorIndirectX),
            0x51 => Some(Self::XorIndirectY),
            0x29 => Some(Self::AndImm),
            0x25 => Some(Self::AndZeroPage),
            0x35 => Some(Self::AndZeroPageX),
            0x2D => Some(Self::AndAbsolute),
            0x3D => Some(Self::AndAbsoluteX),
            0x39 => Some(Self::AndAbsoluteY),
            0x21 => Some(Self::AndIndirectX),
            0x31 => Some(Self::AndIndirectY),
            _ => None,
        }
    }
}
