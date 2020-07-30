use crate::opcodes::*;
use crate::{VirtualMachine, I5_7360U, SECOND_TO_NANOSECOND};
use std::collections::HashMap;
use std::fs;

struct Mvm1 {
    ctx: Context,
    cycles: i64,
}

impl Mvm1 {
    fn new(memory_bytes: usize) -> Self {
        Mvm1 {
            ctx: Context::new(memory_bytes),
            cycles: 0,
        }
    }

    fn run(&mut self, application: &Application) -> Result<(), String> {
        while self.ctx.pc / 4 < application.instructions.len() as i32 {
            let idx = self.fetch_instruction();
            let runner = &application.instructions[idx];
            self.decode(runner);
            self.execute_write(application, runner)?;
        }
        let s = self.cycles as f64 / I5_7360U as f64;
        let ns = s * SECOND_TO_NANOSECOND as f64;
        println!("{} cycles, {} seconds, {} nanoseconds", self.cycles, s, ns);
        return Ok(());
    }

    fn fetch_instruction(&mut self) -> usize {
        self.cycles += 50;
        (self.ctx.pc / 4) as usize
    }

    fn decode(&mut self, runner: &Box<dyn InstructionRunner>) -> InstructionType {
        self.cycles += 1;
        runner.instruction_type()
    }

    fn execute_write(
        &mut self,
        application: &Application,
        runner: &Box<dyn InstructionRunner>,
    ) -> Result<(), String> {
        runner.run(&mut self.ctx, &application.labels)?;

        let cycles = match runner.instruction_type() {
            InstructionType::ADD => 2,
            InstructionType::ADDI => 2,
            InstructionType::AND => 2,
            InstructionType::ANDI => 2,
            InstructionType::AUIPC => 2,
            InstructionType::BEQ => 2,
            InstructionType::BGE => 2,
            InstructionType::BGEU => 2,
            InstructionType::BLT => 2,
            InstructionType::BLTU => 2,
            InstructionType::BNE => 2,
            InstructionType::DIV => 2,
            InstructionType::JAL => 2,
            InstructionType::JALR => 2,
            InstructionType::LUI => 2,
            InstructionType::LB => 51,
            InstructionType::LH => 51,
            InstructionType::LW => 51,
            InstructionType::NOP => 2,
            InstructionType::MUL => 2,
            InstructionType::OR => 2,
            InstructionType::ORI => 2,
            InstructionType::REM => 2,
            InstructionType::SB => 50,
            InstructionType::SH => 50,
            InstructionType::SLL => 2,
            InstructionType::SLLI => 2,
            InstructionType::SLT => 2,
            InstructionType::SLTU => 2,
            InstructionType::SLTI => 2,
            InstructionType::SRA => 2,
            InstructionType::SRAI => 2,
            InstructionType::SRL => 2,
            InstructionType::SRLI => 2,
            InstructionType::SUB => 2,
            InstructionType::SW => 50,
            InstructionType::XOR => 2,
            InstructionType::XORI => 2,
        };
        self.cycles += cycles;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bit::bytes_from_low_bits;
    use crate::parser::parse;
    use std::borrow::Borrow;

    macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

    fn assert(
        init_registers: HashMap<RegisterType, i32>,
        memory_bytes: usize,
        init_memory: HashMap<usize, i8>,
        instructions: &str,
        assertions_registers: HashMap<RegisterType, i32>,
        assertions_memory: HashMap<usize, i8>,
    ) {
        let application = parse(instructions.to_string()).unwrap();
        let mut runner = Mvm1::new(memory_bytes);
        for register in init_registers {
            runner.ctx.registers[register.0] = register.1;
        }
        for memory in init_memory {
            runner.ctx.memory[memory.0] = memory.1;
        }
        runner.run(&application).unwrap();
        for assertion in assertions_registers {
            assert_eq!(runner.ctx.registers[assertion.0], assertion.1);
        }
        for assertion in assertions_memory {
            assert_eq!(runner.ctx.memory[assertion.0], assertion.1);
        }
    }

    #[test]
    fn test_prime_number() {
        let bits = bytes_from_low_bits(1109);
        assert(
            HashMap::new(),
            5,
            map! {0 => bits.0,1 => bits.1,2 => bits.2,3 => bits.3},
            fs::read_to_string("res/risc/prime-number.asm")
                .unwrap()
                .as_str()
                .borrow(),
            map! {RegisterType::A0 => 4},
            map! {4=>1},
        );
    }
}
