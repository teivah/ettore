use crate::opcodes::*;
use crate::VirtualMachine;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;

const CYCLES_MEMORY_ACCESS: f32 = 50.;
const CYCLES_L1_ACCESS: f32 = 1.;
const CYCLES_REGISTER_ACCESS: f32 = 1.;
const CYCLES_DECODE: f32 = 1.;

pub struct Mvm3 {
    ctx: Context,
    cycles: f32,

    fetch_instruction: FetchInstruction,
    fetch_instruction_out: Option<usize>,
}

impl VirtualMachine for Mvm3 {
    fn run(&mut self, application: &Application) -> Result<f32, String> {
        let mut cycles: f32 = 0.;
        loop {
            cycles += 1.;
            let instruction = self
                .fetch_instruction
                .cycle(application, self.fetch_instruction_out.is_some());
            if instruction.is_some() {
                self.fetch_instruction_out = Some(instruction.unwrap());
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
            fetch_instruction: FetchInstruction::new(),
            fetch_instruction_out: None,
        }
    }
}

struct L1I {
    boundary: (i32, i32),
}

impl L1I {
    fn present(&self, pc: i32) -> bool {
        pc >= self.boundary.0 && pc <= self.boundary.1
    }

    fn fetch(&mut self, pc: i32) {
        self.boundary = (pc, pc + 64);
    }
}

struct FetchInstruction {
    pc: i32,
    l1i: L1I,
    remaining_cycles: f32,
    complete: bool,
    processing: bool,
}

impl FetchInstruction {
    fn new() -> Self {
        FetchInstruction {
            pc: 0,
            l1i: L1I { boundary: (-1, -1) },
            remaining_cycles: 0.0,
            complete: false,
            processing: false,
        }
    }

    fn cycle(&mut self, application: &Application, output_bus_blocked: bool) -> Option<usize> {
        if self.complete {
            return None;
        }

        if !self.processing {
            self.processing = true;
            if self.l1i.present(self.pc) {
                self.remaining_cycles = CYCLES_L1_ACCESS;
            } else {
                self.remaining_cycles = CYCLES_MEMORY_ACCESS;
                // Should be done after the processing of the 50 cycles
                self.l1i.fetch(self.pc);
                return None;
            }
        }

        self.remaining_cycles -= 1.;
        return if self.remaining_cycles == 0. {
            if output_bus_blocked {
                self.remaining_cycles = 1.;
                return None;
            }

            self.processing = false;
            let current_pc = self.pc;
            self.pc += 4;
            if self.pc / 4 >= application.instructions.len() as i32 {
                self.complete = true;
            }
            Some((current_pc / 4) as usize)
        } else {
            None
        };
    }

    fn set_pc(&mut self, pc: i32) {
        self.pc = pc;
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
