use crate::opcodes::*;
use crate::VirtualMachine;
use std::collections::HashMap;
use std::fs;

const CYCLES_MEMORY_ACCESS: f32 = 50.;
const CYCLES_L1_ACCESS: f32 = 1.;
const CYCLES_REGISTER_ACCESS: f32 = 1.;
const CYCLES_DECODE: f32 = 1.;

pub struct Mvm3 {
    ctx: Context,
    cycles: f32,
    l1i: (i32, i32),
}

struct Bus<T> {
    value: Option<T>,
}

impl VirtualMachine for Mvm3 {
    fn run(&mut self, application: &Application) -> Result<f32, String> {
        while self.ctx.pc / 4 < application.instructions.len() as i32 {
            let idx = self.fetch_instruction();
            let runner = &application.instructions[idx];
            self.decode(runner);
            let instruction_type = self.execute(application, runner)?;
            if write_back(instruction_type) {
                self.write();
            }
        }
        return Ok(self.cycles);
    }
}

impl Mvm3 {
    pub fn new(memory_bytes: usize) -> Self {
        Mvm3 {
            ctx: Context::new(memory_bytes),
            cycles: 0.,
            l1i: (-1, -1),
        }
    }

    fn fetch_instruction(&mut self) -> usize {
        if self.present_in_l1i() {
            self.cycles += CYCLES_L1_ACCESS;
        } else {
            self.fetch_l1i();
        }

        (self.ctx.pc / 4) as usize
    }

    fn present_in_l1i(&self) -> bool {
        self.ctx.pc >= self.l1i.0 && self.ctx.pc <= self.l1i.1
    }

    fn fetch_l1i(&mut self) {
        self.cycles += CYCLES_MEMORY_ACCESS;
        self.l1i = (self.ctx.pc, self.ctx.pc + 64);
    }

    fn decode(&mut self, runner: &Box<dyn InstructionRunner>) -> InstructionType {
        self.cycles += CYCLES_DECODE;
        runner.instruction_type()
    }

    fn execute(
        &mut self,
        application: &Application,
        runner: &Box<dyn InstructionRunner>,
    ) -> Result<InstructionType, String> {
        runner.run(&mut self.ctx, &application.labels)?;
        self.cycles += cycles_per_instruction(runner.instruction_type());
        Ok(runner.instruction_type())
    }

    fn write(&mut self) {
        self.cycles += CYCLES_REGISTER_ACCESS;
    }
}

struct Fetch {}

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
        let mut runner = Mvm3::new(memory_bytes);
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

    #[test]
    fn test_pipelining() {
        assert(
            HashMap::new(),
            5,
            HashMap::new(),
            "addi t0, zero, 1
            addi t1, zero, 2
            addi t2, zero, 3",
            map! {RegisterType::T0=> 1, RegisterType::T1 => 2, RegisterType::T2 => 3 },
            HashMap::new(),
        );
    }
}
