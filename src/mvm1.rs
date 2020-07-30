use crate::opcodes::{Application, Context, RegisterType};
use std::collections::HashMap;
use std::fs;

struct Runner {
    ctx: Context,
    application: Application,
    cycles: i32,
}

impl Runner {
    fn new(application: Application, memory_bytes: usize) -> Self {
        Runner {
            ctx: Context::new(memory_bytes),
            application,
            cycles: 0,
        }
    }

    fn run(&mut self) -> Result<(), String> {
        while self.ctx.pc / 4 < self.application.instructions.len() as i32 {
            self.fetch_instruction();
            self.decode();
            self.execute_write((self.ctx.pc / 4) as usize)?;
        }
        return Ok(());
    }

    fn fetch_instruction(&mut self) {
        self.cycles += 50;
    }

    fn decode(&mut self) {
        self.cycles += 1;
    }

    fn execute_write(&mut self, idx: usize) -> Result<(), String> {
        let runner = &self.application.instructions[idx];
        return runner.run(&mut self.ctx, &self.application.labels);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let mut runner = Runner::new(application, memory_bytes);
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
        assert(
            HashMap::new(),
            5,
            map! {0 => 9},
            fs::read_to_string("res/risc/prime-number.asm")
                .unwrap()
                .as_str()
                .borrow(),
            map! {RegisterType::A0 => 4},
            map! {4=>0},
        );

        assert(
            HashMap::new(),
            5,
            map! {0 => 13},
            fs::read_to_string("res/risc/prime-number.asm")
                .unwrap()
                .as_str()
                .borrow(),
            map! {RegisterType::A0 => 4},
            map! {4=>1},
        );
    }
}
