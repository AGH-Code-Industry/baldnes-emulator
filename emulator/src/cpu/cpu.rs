use crate::bus::BusLike;
use crate::cpu::micro_instructions::{MicroInstruction, MicroInstructionSequence};
use crate::cpu::registers::Registers;

#[allow(dead_code)]
pub struct CPU<T: BusLike> {
    bus: T,
    registers: Registers,
    state: CPUState,
    fetching_operation: MicroInstructionSequence,
    current_micro_instruction: Option<MicroInstruction>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum CPUFlag {
    CarryBit,
    Zero,
    InterruptDisable,
    DecimalMode,
    Break,
    Unused,
    Overflow,
    Negative,
}

#[derive(PartialEq, Debug)]
pub enum CPUState {
    Fetching,
    Execution,
}
#[allow(dead_code)]
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
            MicroInstruction::ReadZeroPageBalY => {
                self.registers.read_zero_page_bal_y(&mut self.bus);
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
            MicroInstruction::WriteZeroPageBalX => {
                self.registers.write_zero_page_bal_x(&mut self.bus)
            }
            MicroInstruction::WriteZeroPageBalY => {
                self.registers.write_zero_page_bal_y(&mut self.bus);
            }
            MicroInstruction::WriteAbsolute => self.registers.write_absolute(&mut self.bus),
            MicroInstruction::ClearCarryFlag => self.registers.clear_flag(CPUFlag::CarryBit),
            MicroInstruction::ClearDecimalFlag => self.registers.clear_flag(CPUFlag::DecimalMode),
            MicroInstruction::ClearInterruptDisableFlag => {
                self.registers.clear_flag(CPUFlag::InterruptDisable)
            }
            MicroInstruction::ClearOverflowFlag => self.registers.clear_flag(CPUFlag::Overflow),
            MicroInstruction::SetCarryFlag => self.registers.set_flag(CPUFlag::CarryBit),
            MicroInstruction::SetDecimalFlag => self.registers.set_flag(CPUFlag::DecimalMode),
            MicroInstruction::SetInterruptDisableFlag => {
                self.registers.set_flag(CPUFlag::InterruptDisable)
            }
            MicroInstruction::ShiftLeftAccumulator => self.registers.shift_left_accumulator(),
            MicroInstruction::ShiftLeftMemoryBuffer => self.registers.shift_left_memory_buffer(),
            MicroInstruction::PushAccumulator => self.registers.push_accumulator(&mut self.bus),
            MicroInstruction::PushStatusRegister => {
                self.registers.push_status_register(&mut self.bus)
            }
            MicroInstruction::PullAccumulator => self.registers.pull_accumulator(&mut self.bus),
            MicroInstruction::PullStatusRegister => {
                self.registers.pull_status_register(&mut self.bus)
            }
            MicroInstruction::IncrementMemoryBuffer => self.registers.increment_memory_buffer(),
            MicroInstruction::IncrementX => self.registers.increment_x(),
            MicroInstruction::IncrementY => self.registers.increment_y(),
            MicroInstruction::DecrementMemoryBuffer => self.registers.dec_memory_buffer(),
            MicroInstruction::DecrementX => self.registers.dec_x(),
            MicroInstruction::DecrementY => self.registers.dec_y(),
            MicroInstruction::LoadAccumulator => self.registers.load_accumulator(),
            MicroInstruction::LoadX => self.registers.load_x(),
            MicroInstruction::LoadY => self.registers.load_y(),
            MicroInstruction::StoreAccumulator => self.registers.store_accumulator(),
            MicroInstruction::StoreX => self.registers.store_x(),
            MicroInstruction::StoreY => self.registers.store_y(),
            MicroInstruction::TransferAccumulatorToX => self.registers.transfer_acc_to_x(),
            MicroInstruction::TransferAccumulatorToY => self.registers.transfer_acc_to_y(),
            MicroInstruction::TransferStackptrToX => self.registers.transer_stackptr_to_x(),
            MicroInstruction::TransferXToAccumulator => self.registers.transfer_x_to_acc(),
            MicroInstruction::TransferXToStackptr => self.registers.transfer_x_to_stackptr(),
            MicroInstruction::TransferYToAccumulator => self.registers.transfer_y_to_acc(),
            MicroInstruction::And => self.registers.and(),
            MicroInstruction::Xor => self.registers.xor(),
            MicroInstruction::Or => self.registers.or(),
        }
    }
}

impl CPUFlag {
    pub fn value(&self) -> u8 {
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
    use crate::cpu::operations::Operation;

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

    fn _test_read_and_decode_operation(cpu: &mut CPU<TestBus>) {
        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadOperationCode)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::DecodeOperation)
        );
    }

    fn _test_immediate_read(cpu: &mut CPU<TestBus>) {
        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ImmediateRead)
        );
    }

    fn _test_zero_page_read(cpu: &mut CPU<TestBus>) {
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
    }

    fn _test_zero_page_x_read(cpu: &mut CPU<TestBus>) {
        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadBal)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Empty));

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadZeroPageBalX)
        );
    }

    fn _test_zero_page_y_read(cpu: &mut CPU<TestBus>) {
        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadBal)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Empty));

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadZeroPageBalY)
        );
    }

    fn _test_absolute_read(cpu: &mut CPU<TestBus>) {
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
            Some(MicroInstruction::ReadAdh)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadAbsolute)
        );
    }

    fn _test_absolute_x_read(cpu: &mut CPU<TestBus>) {
        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadBal)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadBah)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadAdlAdhAbsoluteX)
        );
    }

    fn _test_absolute_y_read(cpu: &mut CPU<TestBus>) {
        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadBal)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadBah)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadAdlAdhAbsoluteY)
        );
    }

    fn _test_indirect_x_read(cpu: &mut CPU<TestBus>) {
        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadBal)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Empty));

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadAdlIndirectBal)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadAdhIndirectBal)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadAbsolute)
        );
    }

    fn _test_indirect_y_read(cpu: &mut CPU<TestBus>) {
        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadIal)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadBalIndirectIal)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadBahIndirectIal)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ReadAdlAdhAbsoluteY)
        );
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
    fn test_cpu_clear_carry_flag() {
        let opcode = Operation::ClearCarryFlag.get_opcode();

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.set_flag(CPUFlag::CarryBit);

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ClearCarryFlag)
        );

        assert_eq!(cpu.registers.is_flag_set(CPUFlag::CarryBit), false);
    }

    #[test]
    fn test_cpu_clear_decimal_flag() {
        let opcode = Operation::ClearDecimalFlag.get_opcode();

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.set_flag(CPUFlag::DecimalMode);

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ClearDecimalFlag)
        );

        assert_eq!(cpu.registers.is_flag_set(CPUFlag::DecimalMode), false);
    }

    #[test]
    fn test_cpu_clear_interrupt_disable_flag() {
        let opcode = Operation::ClearInterruptDisableFlag.get_opcode();

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.set_flag(CPUFlag::InterruptDisable);

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ClearInterruptDisableFlag)
        );

        assert_eq!(cpu.registers.is_flag_set(CPUFlag::InterruptDisable), false);
    }

    #[test]
    fn test_cpu_clear_overflow_flag() {
        let opcode = Operation::ClearOverflowFlag.get_opcode();

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.set_flag(CPUFlag::Overflow);

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::ClearOverflowFlag)
        );

        assert_eq!(cpu.registers.is_flag_set(CPUFlag::Overflow), false);
    }

    #[test]
    fn test_cpu_set_carry_flag() {
        let opcode = Operation::SetCarryFlag.get_opcode();

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.clear_flag(CPUFlag::CarryBit);

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::SetCarryFlag)
        );

        assert_eq!(cpu.registers.is_flag_set(CPUFlag::CarryBit), true);
    }

    #[test]
    fn test_cpu_set_decimal_flag() {
        let opcode = Operation::SetDecimalFlag.get_opcode();

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.clear_flag(CPUFlag::DecimalMode);

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::SetDecimalFlag)
        );

        assert_eq!(cpu.registers.is_flag_set(CPUFlag::DecimalMode), true);
    }

    #[test]
    fn test_cpu_set_interrupt_disable_flag() {
        let opcode = Operation::SetInterruptDisableFlag.get_opcode();

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.clear_flag(CPUFlag::InterruptDisable);

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::SetInterruptDisableFlag)
        );

        assert_eq!(cpu.registers.is_flag_set(CPUFlag::InterruptDisable), true);
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

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

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

    #[test]
    fn test_cpu_push_acc() {
        let opcode = Operation::PushAcc.get_opcode();

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = 20;

        _test_read_and_decode_operation(&mut cpu);

        assert_eq!(cpu.registers.stack_ptr, 0xFF);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::PushAccumulator)
        );

        let stack_value: u8 = cpu.bus.read(0x01FF);
        assert_eq!(stack_value, cpu.registers.a);
        assert_eq!(cpu.registers.stack_ptr, 0xFF - 1);
    }

    #[test]
    fn test_cpu_push_status() {
        let opcode = Operation::PushStatus.get_opcode();

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        assert_eq!(cpu.registers.status, 0x00);
        cpu.registers.set_flag(CPUFlag::Negative);
        cpu.registers.set_flag(CPUFlag::CarryBit);
        cpu.registers.set_flag(CPUFlag::Overflow);

        _test_read_and_decode_operation(&mut cpu);

        assert_eq!(cpu.registers.stack_ptr, 0xFF);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::PushStatusRegister)
        );

        let stack_value: u8 = cpu.bus.read(0x01FF);
        assert_eq!(
            stack_value,
            cpu.registers.status | CPUFlag::Break.value() | CPUFlag::Unused.value()
        );
        assert_eq!(cpu.registers.stack_ptr, 0xFF - 1);
    }

    #[test]
    fn test_cpu_pull_acc() {
        let opcode = Operation::PullAcc.get_opcode();

        let mut bus = TestBus::new();
        let stack_value: u8 = 100;
        bus.write(0x0000, opcode);
        bus.write(0x01FF, stack_value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = 20;
        cpu.registers.stack_ptr = 0xFF - 1;

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::PullAccumulator)
        );

        assert_eq!(stack_value, cpu.registers.a);
        assert_eq!(cpu.registers.stack_ptr, 0xFF);
    }

    #[test]
    fn test_cpu_pull_status() {
        let opcode = Operation::PullStatus.get_opcode();

        let mut bus = TestBus::new();
        let stack_status: u8 = 0b1011_1100;
        bus.write(0x0000, opcode);
        bus.write(0x01FF, stack_status);

        let mut cpu = CPU::new(bus);
        cpu.registers.stack_ptr = 0xFF - 1;
        assert_eq!(cpu.registers.status, 0x00);

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::PullStatusRegister)
        );

        assert_eq!(stack_status & 0b1100_1111, cpu.registers.status);
        assert_eq!(cpu.registers.stack_ptr, 0xFF);
    }

    #[test]
    fn test_cpu_inc_mem_zero_page() {
        let opcode: u8 = Operation::IncMemZeroPage.get_opcode();
        let address: u8 = 0xF1;
        let value: u8 = 10;
        let expected_value: u8 = 11;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, address);
        bus.write(address as u16, value);
        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::IncrementMemoryBuffer)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteZeroPage)
        );

        let read_value: u8 = cpu.bus.read(address as u16);
        assert_eq!(read_value, expected_value);
    }

    #[test]
    fn test_cpu_inc_mem_zero_page_x() {
        let opcode: u8 = Operation::IncMemZeroPageX.get_opcode();
        let address: u8 = 0xF1;
        let x_value: u8 = 3;
        let value: u8 = 10;
        let expected_value: u8 = 11;
        let expected_address: u8 = address + x_value;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, address);
        bus.write(expected_address as u16, value);
        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::IncrementMemoryBuffer)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteZeroPageBalX)
        );

        let read_value: u8 = cpu.bus.read(expected_address as u16);
        assert_eq!(read_value, expected_value);
    }

    #[test]
    fn test_cpu_inc_mem_absolute() {
        let opcode: u8 = Operation::IncMemAbsolute.get_opcode();
        let adl: u8 = 0xF1;
        let adh: u8 = 0xFF;
        let address: u16 = 0xFFF1;
        let value: u8 = 10;
        let expected_value: u8 = 11;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(address, value);
        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::IncrementMemoryBuffer)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteAbsolute)
        );

        let read_value = cpu.bus.read(address);
        assert_eq!(read_value, expected_value);
    }

    #[test]
    fn test_cpu_inc_mem_absolute_x() {
        let opcode: u8 = Operation::IncMemAbsoluteX.get_opcode();
        let adl: u8 = 0xF1;
        let adh: u8 = 0xFF;
        let address: u16 = 0xFFF1;
        let value: u8 = 10;
        let expected_value: u8 = 11;
        let x_value: u8 = 5;
        let expected_address = address + x_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);
        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::IncrementMemoryBuffer)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteAbsolute)
        );

        let read_value = cpu.bus.read(expected_address);
        assert_eq!(read_value, expected_value);
    }

    #[test]
    fn test_cpu_inc_x() {
        let opcode = Operation::IncX.get_opcode();
        let x_value: u8 = 30;
        let expected_value: u8 = 31;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::IncrementX)
        );

        assert_eq!(cpu.registers.x, expected_value);
    }

    #[test]
    fn test_cpu_inc_y() {
        let opcode = Operation::IncY.get_opcode();
        let y_value: u8 = 30;
        let expected_value: u8 = 31;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        let mut cpu = CPU::new(bus);
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::IncrementY)
        );

        assert_eq!(cpu.registers.y, expected_value);
    }

    #[test]
    fn test_cpu_dec_mem_zero_page() {
        let opcode: u8 = Operation::DecMemZeroPage.get_opcode();
        let address: u8 = 0xF1;
        let value: u8 = 10;
        let expected_value: u8 = 9;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, address);
        bus.write(address as u16, value);
        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

        println!("{}", cpu.registers.memory_buffer);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::DecrementMemoryBuffer)
        );

        println!("{}", cpu.registers.memory_buffer);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteZeroPage)
        );

        let read_value: u8 = cpu.bus.read(address as u16);
        assert_eq!(read_value, expected_value);
    }

    #[test]
    fn test_cpu_dec_mem_zero_page_x() {
        let opcode: u8 = Operation::DecMemZeroPageX.get_opcode();
        let address: u8 = 0xF1;
        let x_value: u8 = 3;
        let value: u8 = 10;
        let expected_value: u8 = 9;
        let expected_address: u8 = address + x_value;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, address);
        bus.write(expected_address as u16, value);
        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::DecrementMemoryBuffer)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteZeroPageBalX)
        );

        let read_value: u8 = cpu.bus.read(expected_address as u16);
        assert_eq!(read_value, expected_value);
    }

    #[test]
    fn test_cpu_dec_mem_absolute() {
        let opcode: u8 = Operation::DecMemAbsolute.get_opcode();
        let adl: u8 = 0xF1;
        let adh: u8 = 0xFF;
        let address: u16 = 0xFFF1;
        let value: u8 = 10;
        let expected_value: u8 = 9;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(address, value);
        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::DecrementMemoryBuffer)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteAbsolute)
        );

        let read_value = cpu.bus.read(address);
        assert_eq!(read_value, expected_value);
    }

    #[test]
    fn test_cpu_dec_mem_absolute_x() {
        let opcode: u8 = Operation::DecMemAbsoluteX.get_opcode();
        let adl: u8 = 0xF1;
        let adh: u8 = 0xFF;
        let address: u16 = 0xFFF1;
        let value: u8 = 10;
        let expected_value: u8 = 9;
        let x_value: u8 = 5;
        let expected_address = address + x_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);
        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::DecrementMemoryBuffer)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteAbsolute)
        );

        let read_value = cpu.bus.read(expected_address);
        assert_eq!(read_value, expected_value);
    }

    #[test]
    fn test_cpu_dec_x() {
        let opcode = Operation::DecX.get_opcode();
        let x_value: u8 = 30;
        let expected_value: u8 = 29;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::DecrementX)
        );

        assert_eq!(cpu.registers.x, expected_value);
    }

    #[test]
    fn test_cpu_dec_y() {
        let opcode = Operation::DecY.get_opcode();
        let y_value: u8 = 30;
        let expected_value: u8 = 29;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        let mut cpu = CPU::new(bus);
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::DecrementY)
        );

        assert_eq!(cpu.registers.y, expected_value);
    }

    #[test]
    fn test_cpu_load_acc_imm() {
        let opcode = Operation::LoadAccImm.get_opcode();
        let value: u8 = 44;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, value);

        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_immediate_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::LoadAccumulator)
        );

        assert_eq!(cpu.registers.a, value);
    }

    #[test]
    fn test_cpu_load_acc_zero_page() {
        let opcode = Operation::LoadAccZeroPage.get_opcode();
        let adl: u8 = 0x80;
        let value: u8 = 44;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, value);

        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::LoadAccumulator)
        );

        assert_eq!(cpu.registers.a, value);
    }

    #[test]
    fn test_cpu_load_acc_zero_page_x() {
        let opcode = Operation::LoadAccZeroPageX.get_opcode();
        let adl: u8 = 0x80;
        let value: u8 = 44;
        let x_value: u8 = 15;
        let expected_address: u8 = adl + x_value;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address as u16, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::LoadAccumulator)
        );

        assert_eq!(cpu.registers.a, value);
    }

    #[test]
    fn test_cpu_load_acc_absolute() {
        let opcode = Operation::LoadAccAbsolute.get_opcode();
        let adl: u8 = 0x80;
        let adh: u8 = 0xAB;
        let address: u16 = 0xAB80;
        let value: u8 = 44;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(address, value);

        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::LoadAccumulator)
        );
    }

    #[test]
    fn test_cpu_load_acc_absolute_x() {
        let opcode = Operation::LoadAccAbsoluteX.get_opcode();
        let value: u8 = 31;
        let adl: u8 = 0x80;
        let adh: u8 = 0xAA;
        let address: u16 = 0xAA80;
        let x_value: u8 = 10;
        let expected_address: u16 = address + x_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::LoadAccumulator)
        );

        assert_eq!(cpu.registers.a, value);
    }

    #[test]
    fn test_cpu_load_acc_absolute_y() {
        let opcode = Operation::LoadAccAbsoluteY.get_opcode();
        let value: u8 = 31;
        let adl: u8 = 0x80;
        let adh: u8 = 0xAA;
        let address: u16 = 0xAA80;
        let y_value: u8 = 10;
        let expected_address: u16 = address + y_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::LoadAccumulator)
        );

        assert_eq!(cpu.registers.a, value);
    }

    #[test]
    fn test_cpu_load_acc_indirect_x() {
        let opcode = Operation::LoadAccIndirectX.get_opcode();
        let value: u8 = 30;
        let x_value: u8 = 10;
        let adl: u8 = 0x80;
        let expected_address: u16 = (adl + x_value) as u16;
        let indirect_adl: u8 = 0xBB;
        let indirect_adh: u8 = 0xAA;
        let indirect_address: u16 = 0xAABB;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address, indirect_adl);
        bus.write(expected_address + 1, indirect_adh);
        bus.write(indirect_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_indirect_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::LoadAccumulator)
        );

        assert_eq!(cpu.registers.a, value);
    }

    #[test]
    fn test_cpu_load_acc_indirect_y() {
        let opcode = Operation::LoadAccIndirectY.get_opcode();
        let value: u8 = 60;
        let y_value: u8 = 20;
        let adl: u8 = 0x80;
        let indirect_adl: u8 = 0xBB;
        let indirect_adh: u8 = 0xAA;
        let indirect_address: u16 = 0xAABB;
        let expected_address: u16 = indirect_address + y_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, indirect_adl);
        bus.write((adl + 1) as u16, indirect_adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_indirect_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::LoadAccumulator)
        );

        assert_eq!(cpu.registers.a, value);
    }

    #[test]
    fn test_cpu_load_x_imm() {
        let opcode = Operation::LoadXImm.get_opcode();
        let value: u8 = 20;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, value);

        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_immediate_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::LoadX));

        assert_eq!(cpu.registers.x, value);
    }

    #[test]
    fn test_cpu_load_x_zero_page() {
        let opcode = Operation::LoadXZeroPage.get_opcode();
        let adl: u8 = 0x2F;
        let value: u8 = 20;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, value);

        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::LoadX));

        assert_eq!(cpu.registers.x, value);
    }

    #[test]
    fn test_cpu_load_x_zero_page_y() {
        let opcode = Operation::LoadXZeroPageY.get_opcode();
        let adl: u8 = 0x2F;
        let value: u8 = 4;
        let y_value: u8 = 25;
        let expected_address: u16 = (adl + y_value) as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::LoadX));

        assert_eq!(cpu.registers.x, value);
    }

    #[test]
    fn test_cpu_load_x_absolute() {
        let opcode = Operation::LoadXAbsolute.get_opcode();
        let adl: u8 = 0x2F;
        let adh: u8 = 0xBB;
        let value: u8 = 4;
        let address: u16 = 0xBB2F;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(address, value);

        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::LoadX));

        assert_eq!(cpu.registers.x, value);
    }

    #[test]
    fn test_cpu_load_x_absolute_y() {
        let opcode = Operation::LoadXAbsoluteY.get_opcode();
        let adl: u8 = 0x2F;
        let adh: u8 = 0xBB;
        let value: u8 = 4;
        let address: u16 = 0xBB2F;
        let y_value: u8 = 36;
        let expected_address: u16 = address + y_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::LoadX));

        assert_eq!(cpu.registers.x, value);
    }

    #[test]
    fn test_cpu_load_y_imm() {
        let opcode = Operation::LoadYImm.get_opcode();
        let value: u8 = 20;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, value);

        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_immediate_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::LoadY));

        assert_eq!(cpu.registers.y, value);
    }

    #[test]
    fn test_cpu_load_y_zero_page() {
        let opcode = Operation::LoadYZeroPage.get_opcode();
        let adl: u8 = 0x2F;
        let value: u8 = 20;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, value);

        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::LoadY));

        assert_eq!(cpu.registers.y, value);
    }

    #[test]
    fn test_cpu_load_y_zero_page_x() {
        let opcode = Operation::LoadYZeroPageX.get_opcode();
        let adl: u8 = 0x2F;
        let value: u8 = 4;
        let x_value: u8 = 25;
        let expected_address: u16 = (adl + x_value) as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::LoadY));

        assert_eq!(cpu.registers.y, value);
    }

    #[test]
    fn test_cpu_load_y_absolute() {
        let opcode = Operation::LoadYAbsolute.get_opcode();
        let adl: u8 = 0x2F;
        let adh: u8 = 0xBB;
        let value: u8 = 4;
        let address: u16 = 0xBB2F;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(address, value);

        let mut cpu = CPU::new(bus);

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::LoadY));

        assert_eq!(cpu.registers.y, value);
    }

    #[test]
    fn test_cpu_load_y_absolute_x() {
        let opcode = Operation::LoadYAbsoluteX.get_opcode();
        let adl: u8 = 0x2F;
        let adh: u8 = 0xBB;
        let value: u8 = 4;
        let address: u16 = 0xBB2F;
        let x_value: u8 = 36;
        let expected_address: u16 = address + x_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::LoadY));

        assert_eq!(cpu.registers.y, value);
    }

    #[test]
    fn test_cpu_store_acc_zero_page() {
        let opcode = Operation::StoreAccZeroPage.get_opcode();
        let adl: u8 = 0xAA;
        let acc_value: u8 = 3;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreAccumulator)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteZeroPage)
        );

        let value = cpu.bus.read(adl as u16);
        assert_eq!(value, cpu.registers.a);
    }

    #[test]
    fn test_cpu_store_acc_zero_page_x() {
        let opcode = Operation::StoreAccZeroPageX.get_opcode();
        let adl: u8 = 0x0B;
        let x_value: u8 = 30;
        let acc_value: u8 = 100;
        let expected_address: u16 = (adl + x_value) as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreAccumulator)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteZeroPageBalX)
        );

        let value = cpu.bus.read(expected_address);
        assert_eq!(value, cpu.registers.a)
    }

    #[test]
    fn test_cpu_store_acc_absolute() {
        let opcode = Operation::StoreAccAbsolute.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let acc_value: u8 = 10;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(address, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreAccumulator)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteAbsolute)
        );

        let value = cpu.bus.read(address);
        assert_eq!(value, cpu.registers.a);
    }

    #[test]
    fn test_cpu_store_acc_absolute_x() {
        let opcode = Operation::StoreAccAbsoluteX.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let acc_value: u8 = 10;
        let x_value: u8 = 20;
        let expected_address: u16 = address + x_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreAccumulator)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteAbsolute)
        );

        let value = cpu.bus.read(expected_address);
        assert_eq!(value, cpu.registers.a);
    }

    #[test]
    fn test_cpu_store_acc_absolute_y() {
        let opcode = Operation::StoreAccAbsoluteY.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let acc_value: u8 = 10;
        let y_value: u8 = 20;
        let expected_address: u16 = address + y_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreAccumulator)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteAbsolute)
        );

        let value = cpu.bus.read(expected_address);
        assert_eq!(value, cpu.registers.a);
    }

    #[test]
    fn test_cpu_store_acc_indirect_x() {
        let opcode = Operation::StoreAccIndirectX.get_opcode();
        let a_value: u8 = 5;
        let x_value: u8 = 10;
        let adl: u8 = 0xA0;
        let expected_address: u16 = (adl + x_value) as u16;
        let indirect_adl: u8 = 0x88;
        let indirect_adh: u8 = 0xAA;
        let indirect_address: u16 = 0xAA88;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address, indirect_adl);
        bus.write(expected_address + 1, indirect_adh);
        bus.write(indirect_address, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_indirect_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreAccumulator)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteAbsolute)
        );

        let value = cpu.bus.read(indirect_address);
        assert_eq!(value, cpu.registers.a);
    }

    #[test]
    fn test_cpu_store_acc_indirect_y() {
        let opcode = Operation::StoreAccIndirectY.get_opcode();
        let acc_value: u8 = 13;
        let y_value: u8 = 20;
        let adl: u8 = 0x22;
        let indirect_adl: u8 = 0xBB;
        let indirect_adh: u8 = 0xAA;
        let indirect_address: u16 = 0xAABB;
        let expected_address: u16 = indirect_address + y_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, indirect_adl);
        bus.write((adl + 1) as u16, indirect_adh);
        bus.write(expected_address, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_indirect_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreAccumulator)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteAbsolute)
        );

        let value = cpu.bus.read(expected_address);
        assert_eq!(value, cpu.registers.a);
    }

    #[test]
    fn test_cpu_store_x_zero_page() {
        let opcode = Operation::StoreXZeroPage.get_opcode();
        let adl: u8 = 0xAA;
        let x_value: u8 = 5;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreX)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteZeroPage)
        );

        let value = cpu.bus.read(adl as u16);
        assert_eq!(value, cpu.registers.x);
    }

    #[test]
    fn test_cpu_store_x_zero_page_y() {
        let opcode = Operation::StoreXZeroPageY.get_opcode();
        let adl: u8 = 0x0B;
        let x_value: u8 = 30;
        let y_value: u8 = 10;
        let expected_address: u16 = (adl + y_value) as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreX)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteZeroPageBalY)
        );

        let value = cpu.bus.read(expected_address);
        assert_eq!(value, cpu.registers.x)
    }

    #[test]
    fn test_cpu_store_x_absolute() {
        let opcode = Operation::StoreXAbsolute.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let x_value: u8 = 10;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(address, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreX)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteAbsolute)
        );

        let value = cpu.bus.read(address);
        assert_eq!(value, cpu.registers.x);
    }

    #[test]
    fn test_cpu_store_y_zero_page() {
        let opcode = Operation::StoreYZeroPage.get_opcode();
        let adl: u8 = 0xAA;
        let y_value: u8 = 5;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreY)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteZeroPage)
        );

        let value = cpu.bus.read(adl as u16);
        assert_eq!(value, cpu.registers.y);
    }

    #[test]
    fn test_cpu_store_y_zero_page_x() {
        let opcode = Operation::StoreYZeroPageX.get_opcode();
        let adl: u8 = 0x0B;
        let x_value: u8 = 30;
        let y_value: u8 = 10;
        let expected_address: u16 = (adl + x_value) as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.x = x_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreY)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteZeroPageBalX)
        );

        let value = cpu.bus.read(expected_address);
        assert_eq!(value, cpu.registers.y)
    }

    #[test]
    fn test_cpu_store_y_absolute() {
        let opcode = Operation::StoreYAbsolute.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let y_value: u8 = 10;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(address, 0);

        let mut cpu = CPU::new(bus);
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Execution);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::StoreY)
        );

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::WriteAbsolute)
        );

        let value = cpu.bus.read(address);
        assert_eq!(value, cpu.registers.y);
    }

    #[test]
    fn test_cpu_transfer_acc_to_x() {
        let opcode = Operation::TransferAccToX.get_opcode();
        let acc_value: u8 = 10;
        let x_value: u8 = 5;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::TransferAccumulatorToX)
        );

        assert_eq!(cpu.registers.x, cpu.registers.a);
    }

    #[test]
    fn test_cpu_transfer_acc_to_y() {
        let opcode = Operation::TransferAccToY.get_opcode();
        let acc_value: u8 = 10;
        let y_value: u8 = 5;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::TransferAccumulatorToY)
        );

        assert_eq!(cpu.registers.y, cpu.registers.a);
    }

    #[test]
    fn test_cpu_transfer_stackptr_to_x() {
        let opcode = Operation::TransferStackptrToX.get_opcode();
        let stack_ptr_value: u8 = 0xAB;
        let x_value: u8 = 10;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.stack_ptr = stack_ptr_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::TransferStackptrToX)
        );

        assert_eq!(cpu.registers.x, cpu.registers.stack_ptr);
    }

    #[test]
    fn test_cpu_transfer_x_to_acc() {
        let opcode = Operation::TransferXToAcc.get_opcode();
        let acc_value: u8 = 21;
        let x_value: u8 = 10;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::TransferXToAccumulator)
        );

        assert_eq!(cpu.registers.a, cpu.registers.x);
    }

    #[test]
    fn test_cpu_transfer_x_to_stackptr() {
        let opcode = Operation::TransferXToStackptr.get_opcode();
        let stack_ptr_value: u8 = 0x12;
        let x_value: u8 = 10;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.stack_ptr = stack_ptr_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::TransferXToStackptr)
        );

        assert_eq!(cpu.registers.stack_ptr, cpu.registers.x);
    }

    #[test]
    fn test_cpu_transfer_y_to_acc() {
        let opcode = Operation::TransferYToAcc.get_opcode();
        let acc_value: u8 = 133;
        let y_value: u8 = 10;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(
            cpu.current_micro_instruction,
            Some(MicroInstruction::TransferYToAccumulator)
        );

        assert_eq!(cpu.registers.a, cpu.registers.y);
    }

    #[test]
    fn test_cpu_and_imm() {
        let opcode = Operation::AndImm.get_opcode();
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let expected_value: u8 = 0b0000_0010;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_immediate_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::And));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_and_zero_page() {
        let opcode = Operation::AndZeroPage.get_opcode();
        let adl: u8 = 0xAA;
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let expected_value: u8 = 0b0000_0010;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::And));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_and_zero_page_x() {
        let opcode = Operation::AndZeroPageX.get_opcode();
        let adl: u8 = 0xAA;
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let x_value: u8 = 3;
        let expected_value: u8 = 0b0000_0010;
        let expected_address: u8 = adl + x_value;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address as u16, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::And));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_and_absolute() {
        let opcode = Operation::AndAbsolute.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let expected_value: u8 = 0b0000_0010;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::And));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_and_absolute_x() {
        let opcode = Operation::AndAbsoluteX.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let x_value: u8 = 2;
        let expected_value: u8 = 0b0000_0010;
        let expected_address: u16 = address + x_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::And));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_and_absolute_y() {
        let opcode = Operation::AndAbsoluteY.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let y_value: u8 = 200;
        let expected_value: u8 = 0b0000_0010;
        let expected_address: u16 = address + y_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::And));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_and_indirect_x() {
        let opcode = Operation::AndIndirectX.get_opcode();
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let expected_value: u8 = 0b0000_0010;
        let x_value: u8 = 10;
        let adl: u8 = 0x22;
        let expected_address: u16 = (adl + x_value) as u16;
        let indirect_adl: u8 = 0xBB;
        let indirect_adh: u8 = 0xAA;
        let indirect_address: u16 = 0xAABB;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address, indirect_adl);
        bus.write(expected_address + 1, indirect_adh);
        bus.write(indirect_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_indirect_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::And));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_and_indirect_y() {
        let opcode = Operation::AndIndirectY.get_opcode();
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let expected_value: u8 = 0b0000_0010;
        let y_value: u8 = 20;
        let adl: u8 = 0x22;
        let indirect_adl: u8 = 0xBB;
        let indirect_adh: u8 = 0xAA;
        let indirect_address: u16 = 0xAABB;
        let expected_address: u16 = indirect_address + y_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, indirect_adl);
        bus.write((adl + 1) as u16, indirect_adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_indirect_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::And));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_xor_imm() {
        let opcode = Operation::XorImm.get_opcode();
        let value: u8 = 0b1010_1010;
        let acc_value: u8 = 0b0101_1010;
        let expected_value: u8 = 0b1111_0000;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_immediate_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Xor));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_xor_zero_page() {
        let opcode = Operation::XorZeroPage.get_opcode();
        let adl: u8 = 0xAA;
        let value: u8 = 0b1010_1010;
        let acc_value: u8 = 0b0101_1010;
        let expected_value: u8 = 0b1111_0000;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Xor));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_xor_zero_page_x() {
        let opcode = Operation::XorZeroPageX.get_opcode();
        let adl: u8 = 0xAA;
        let value: u8 = 0b1010_1010;
        let acc_value: u8 = 0b0101_1010;
        let x_value: u8 = 40;
        let expected_address: u8 = adl + x_value;
        let expected_value: u8 = 0b1111_0000;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address as u16, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Xor));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_xor_absolute() {
        let opcode = Operation::XorAbsolute.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let expected_value: u8 = 0b1111_1001;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Xor));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_xor_absolute_x() {
        let opcode = Operation::XorAbsoluteX.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let x_value: u8 = 2;
        let expected_value: u8 = 0b1111_1001;
        let expected_address: u16 = address + x_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Xor));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_xor_absolute_y() {
        let opcode = Operation::XorAbsoluteY.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let y_value: u8 = 200;
        let expected_value: u8 = 0b1111_1001;
        let expected_address: u16 = address + y_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Xor));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_xor_indirect_x() {
        let opcode = Operation::XorIndirectX.get_opcode();
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let expected_value: u8 = 0b1111_1001;
        let x_value: u8 = 10;
        let adl: u8 = 0x22;
        let expected_address: u16 = (adl + x_value) as u16;
        let indirect_adl: u8 = 0xBB;
        let indirect_adh: u8 = 0xAA;
        let indirect_address: u16 = 0xAABB;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address, indirect_adl);
        bus.write(expected_address + 1, indirect_adh);
        bus.write(indirect_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_indirect_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Xor));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_xor_indirect_y() {
        let opcode = Operation::XorIndirectY.get_opcode();
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let expected_value: u8 = 0b1111_1001;
        let y_value: u8 = 20;
        let adl: u8 = 0x22;
        let indirect_adl: u8 = 0xBB;
        let indirect_adh: u8 = 0xAA;
        let indirect_address: u16 = 0xAABB;
        let expected_address: u16 = indirect_address + y_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, indirect_adl);
        bus.write((adl + 1) as u16, indirect_adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_indirect_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Xor));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_or_imm() {
        let opcode = Operation::OrImm.get_opcode();
        let value: u8 = 0b1010_1010;
        let acc_value: u8 = 0b0101_1010;
        let expected_value: u8 = 0b1111_1010;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_immediate_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Or));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_or_zero_page() {
        let opcode = Operation::OrZeroPage.get_opcode();
        let adl: u8 = 0xAA;
        let value: u8 = 0b1010_1010;
        let acc_value: u8 = 0b0101_1010;
        let expected_value: u8 = 0b1111_1010;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Or));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_or_zero_page_x() {
        let opcode = Operation::OrZeroPageX.get_opcode();
        let adl: u8 = 0xAA;
        let value: u8 = 0b1010_1010;
        let acc_value: u8 = 0b0101_1010;
        let x_value: u8 = 40;
        let expected_address: u8 = adl + x_value;
        let expected_value: u8 = 0b1111_1010;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address as u16, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = acc_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_zero_page_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Or));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_or_absolute() {
        let opcode = Operation::OrAbsolute.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let expected_value: u8 = 0b1111_1011;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Or));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_or_absolute_x() {
        let opcode = Operation::OrAbsoluteX.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let x_value: u8 = 2;
        let expected_value: u8 = 0b1111_1011;
        let expected_address: u16 = address + x_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Or));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_or_absolute_y() {
        let opcode = Operation::OrAbsoluteY.get_opcode();
        let adl: u8 = 0xAA;
        let adh: u8 = 0x11;
        let address: u16 = 0x11AA;
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let y_value: u8 = 200;
        let expected_value: u8 = 0b1111_1011;
        let expected_address: u16 = address + y_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(0x0002, adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_absolute_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Or));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_or_indirect_x() {
        let opcode = Operation::OrIndirectX.get_opcode();
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let expected_value: u8 = 0b1111_1011;
        let x_value: u8 = 10;
        let adl: u8 = 0x22;
        let expected_address: u16 = (adl + x_value) as u16;
        let indirect_adl: u8 = 0xBB;
        let indirect_adh: u8 = 0xAA;
        let indirect_address: u16 = 0xAABB;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(expected_address, indirect_adl);
        bus.write(expected_address + 1, indirect_adh);
        bus.write(indirect_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.x = x_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_indirect_x_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Or));

        assert_eq!(cpu.registers.a, expected_value);
    }

    #[test]
    fn test_cpu_or_indirect_y() {
        let opcode = Operation::OrIndirectY.get_opcode();
        let value: u8 = 0b0000_1010;
        let a_value: u8 = 0b1111_0011;
        let expected_value: u8 = 0b1111_1011;
        let y_value: u8 = 20;
        let adl: u8 = 0x22;
        let indirect_adl: u8 = 0xBB;
        let indirect_adh: u8 = 0xAA;
        let indirect_address: u16 = 0xAABB;
        let expected_address: u16 = indirect_address + y_value as u16;

        let mut bus = TestBus::new();
        bus.write(0x0000, opcode);
        bus.write(0x0001, adl);
        bus.write(adl as u16, indirect_adl);
        bus.write((adl + 1) as u16, indirect_adh);
        bus.write(expected_address, value);

        let mut cpu = CPU::new(bus);
        cpu.registers.a = a_value;
        cpu.registers.y = y_value;

        _test_read_and_decode_operation(&mut cpu);

        _test_indirect_y_read(&mut cpu);

        cpu.step();

        assert_eq!(cpu.state, CPUState::Fetching);
        assert_eq!(cpu.current_micro_instruction, Some(MicroInstruction::Or));

        assert_eq!(cpu.registers.a, expected_value);
    }
}
