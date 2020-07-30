use crate::opcodes::*;
use crate::{VirtualMachine, I5_7360U, SECOND_TO_NANOSECOND};
use std::collections::HashMap;
use std::fs;

struct Mvm2 {
    ctx: Context,
    application: Application,
    l1_min: i32,
    l1_max: i32,
}

impl VirtualMachine for Mvm2 {
    fn run(&mut self) -> Result<(), String> {
        let mut cycles: i64 = 0;

        while self.ctx.pc / 4 < self.application.instructions.len() as i32 {
            let fetch = self.fetch_instruction();
            cycles += fetch.1;
            let runner = &self.application.instructions[fetch.0];

            let decode = self.decode(runner);
            cycles += decode.1;
            let ew = Self::execute_write(&mut self.ctx, &self.application.labels, runner)?;
            cycles += ew;
        }

        let s = cycles as f64 / I5_7360U as f64;
        let ns = s * SECOND_TO_NANOSECOND as f64;
        println!("{} cycles, {} seconds, {} nanoseconds", cycles, s, ns);
        return Ok(());
    }
}

impl Mvm2 {
    fn new(application: Application, memory_bytes: usize) -> Self {
        Mvm2 {
            ctx: Context::new(memory_bytes),
            application,
            l1_min: -1,
            l1_max: -1,
        }
    }

    fn fetch_instruction<'a>(&mut self) -> (usize, i64) {
        if self.ctx.pc >= self.l1_min && self.ctx.pc <= self.l1_max {
            return ((self.ctx.pc / 4) as usize, 1);
        }

        self.l1_min = self.ctx.pc;
        self.l1_max = self.ctx.pc + 64;
        return ((self.ctx.pc / 4) as usize, 50);
    }

    fn decode(&self, runner: &Box<dyn InstructionRunner>) -> (InstructionType, i64) {
        (runner.instruction_type(), 1)
    }

    fn execute_write(
        ctx: &mut Context,
        labels: &HashMap<String, i32>,
        runner: &Box<dyn InstructionRunner>,
    ) -> Result<i64, String> {
        runner.run(ctx, labels)?;

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
        Ok(cycles)
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
        let mut runner = Mvm2::new(application, memory_bytes);
        for register in init_registers {
            runner.ctx.registers[register.0] = register.1;
        }
        for memory in init_memory {
            runner.ctx.memory[memory.0] = memory.1;
        }
        runner.run().unwrap();
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
