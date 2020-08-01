use crate::opcodes::*;
use crate::VirtualMachine;
use queues::*;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;

const CYCLES_MEMORY_ACCESS: f32 = 2.;
const CYCLES_L1_ACCESS: f32 = 1.;
const CYCLES_REGISTER_ACCESS: f32 = 1.;
const CYCLES_DECODE: f32 = 1.;

pub struct Mvm3<'a> {
    ctx: Context,
    cycles: f32,

    fetch_unit: FetchUnit,
    decode_bus: Bus<usize>,
    decode_unit: DecodeUnit,
    execute_bus: Bus<&'a Box<dyn InstructionRunner>>,
    execute_unit: ExecuteUnit<'a>,
    write_bus: Bus<InstructionType>,
    write_unit: WriteUnit,
    branch_unit: BranchUnit,
}

pub struct Bus<T: Clone> {
    queue: Queue<T>,
    max: usize,
}

impl<T: Clone> Bus<T> {
    fn new(max: usize) -> Self {
        Bus {
            queue: queue![],
            max,
        }
    }

    fn flush(&mut self) {
        self.queue = queue![];
    }

    fn add(&mut self, t: T) {
        self.queue.add(t).unwrap();
    }

    fn get(&mut self) -> T {
        self.queue.remove().unwrap()
    }

    fn peek(&mut self) -> T {
        self.queue.peek().unwrap()
    }

    fn is_full(&self) -> bool {
        self.queue.size() == self.max
    }

    fn is_empty(&self) -> bool {
        self.queue.size() == 0
    }
}

impl<'a> Mvm3<'a> {
    fn run(&mut self, application: &'a Application) -> Result<f32, String> {
        let mut cycles: f32 = 0.;
        loop {
            cycles += 1.;

            self.fetch_unit.cycle(application, &mut self.decode_bus);
            self.decode_unit
                .cycle(application, &mut self.decode_bus, &mut self.execute_bus);
            if !self.execute_bus.is_empty() {
                let runner = self.execute_bus.peek();
                let instruction_type = runner.instruction_type();
                if jump(&instruction_type) {
                    self.branch_unit.jump();
                } else if conditional_branching(&instruction_type) {
                    self.branch_unit.conditional_branching(self.ctx.pc + 4)
                }
            }
            let execute = self.execute_unit.cycle(
                &mut self.ctx,
                application,
                &mut self.execute_bus,
                &mut self.write_bus,
            )?;
            if execute.is_some() {
                if self.branch_unit.pipeline_to_be_flushed(&self.ctx) {
                    self.flush();
                }
            }
            if !self.write_bus.is_empty() {
                if write_back(self.write_bus.get()) {
                    self.write_unit.cycle();
                }
            }

            if self.is_complete() {
                break;
            }
        }
        return Ok(self.cycles);
    }

    fn flush(&mut self) {
        self.fetch_unit.flush();
        self.decode_bus.flush();
        self.decode_unit.flush();
        self.execute_bus.flush();
        self.write_bus.flush();
    }

    fn is_complete(&self) -> bool {
        self.fetch_unit.is_empty()
            && self.decode_unit.is_empty()
            && self.execute_unit.is_empty()
            && self.write_unit.is_empty()
            && self.decode_bus.is_empty()
            && self.execute_bus.is_empty()
            && self.write_bus.is_empty()
    }
}

impl<'a> Mvm3<'a> {
    pub fn new(memory_bytes: usize) -> Self {
        Mvm3 {
            ctx: Context::new(memory_bytes),
            cycles: 0.,
            fetch_unit: FetchUnit::new(),
            decode_bus: Bus::new(1),
            decode_unit: DecodeUnit::new(),
            execute_bus: Bus::new(1),
            execute_unit: ExecuteUnit::new(),
            write_bus: Bus::new(1),
            write_unit: WriteUnit::new(),
            branch_unit: BranchUnit::new(),
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

struct FetchUnit {
    pc: i32,
    l1i: L1I,
    remaining_cycles: f32,
    complete: bool,
    processing: bool,
}

impl FetchUnit {
    fn new() -> Self {
        FetchUnit {
            pc: 0,
            l1i: L1I { boundary: (-1, -1) },
            remaining_cycles: 0.0,
            complete: false,
            processing: false,
        }
    }

    fn cycle(&mut self, application: &Application, out_bus: &mut Bus<usize>) {
        if self.complete {
            return;
        }

        if !self.processing {
            self.processing = true;
            if self.l1i.present(self.pc) {
                self.remaining_cycles = CYCLES_L1_ACCESS;
            } else {
                self.remaining_cycles = CYCLES_MEMORY_ACCESS;
                // Should be done after the processing of the 50 cycles
                self.l1i.fetch(self.pc);
            }
        }

        self.remaining_cycles -= 1.;
        if self.remaining_cycles == 0. {
            if out_bus.is_full() {
                self.remaining_cycles = 1.;
                return;
            }

            self.processing = false;
            let current_pc = self.pc;
            self.pc += 4;
            if self.pc / 4 >= application.instructions.len() as i32 {
                self.complete = true;
            }
            out_bus.add((current_pc / 4) as usize);
        }
    }

    fn flush(&mut self) {
        self.processing = false;
    }

    fn is_empty(&self) -> bool {
        self.complete
    }
}

struct DecodeUnit {}

impl DecodeUnit {
    fn new() -> Self {
        DecodeUnit {}
    }

    fn cycle<'a>(
        &self,
        application: &'a Application,
        in_bus: &mut Bus<usize>,
        out_bus: &mut Bus<&'a Box<dyn InstructionRunner>>,
    ) {
        if in_bus.is_empty() {
            return;
        }
        let idx = in_bus.get();
        let runner = &application.instructions[idx];
        out_bus.add(runner);
    }

    fn flush(&mut self) {}

    fn is_empty(&self) -> bool {
        // As the decode unit takes only one cycle, it is considered as empty by default
        true
    }
}

struct ExecuteUnit<'a> {
    processing: bool,
    remaining_cycles: f32,
    runner: Option<&'a Box<dyn InstructionRunner>>,
}

impl<'a> ExecuteUnit<'a> {
    fn new() -> Self {
        ExecuteUnit {
            processing: false,
            remaining_cycles: 0.0,
            runner: None,
        }
    }

    fn cycle(
        &mut self,
        ctx: &mut Context,
        application: &Application,
        in_bus: &mut Bus<&'a Box<dyn InstructionRunner>>,
        out_bus: &mut Bus<InstructionType>,
    ) -> Result<Option<()>, String> {
        if !self.processing {
            if in_bus.is_empty() {
                return Ok(None);
            }

            let runner = in_bus.get();
            self.runner = Some(runner);
            self.remaining_cycles = cycles_per_instruction(runner.instruction_type());
            self.processing = true;
        }

        self.remaining_cycles -= 1.;
        if self.remaining_cycles != 0. {
            return Ok(None);
        }

        if out_bus.is_full() {
            self.remaining_cycles = 1.;
            return Ok(None);
        }

        self.processing = false;
        let runner = self.runner.unwrap();
        let pc = runner.run(ctx, &application.labels)?;
        ctx.pc = pc;
        out_bus.add(runner.instruction_type());
        self.runner = None;
        return Ok(Some(()));
    }

    fn flush(&mut self) {}

    fn is_empty(&self) -> bool {
        !self.processing
    }
}

struct WriteUnit {}

impl WriteUnit {
    fn new() -> Self {
        WriteUnit {}
    }

    fn cycle(&self) {}

    fn is_empty(&self) -> bool {
        true
    }
}

struct BranchUnit {
    condition_branching_expected: Option<i32>,
    jump: bool,
}

impl BranchUnit {
    fn new() -> Self {
        BranchUnit {
            condition_branching_expected: None,
            jump: false,
        }
    }

    fn conditional_branching(&mut self, expected: i32) {
        self.condition_branching_expected = Some(expected);
    }

    fn jump(&mut self) {
        self.jump = true;
    }

    fn pipeline_to_be_flushed(&mut self, ctx: &Context) -> bool {
        let mut conditional_branching = false;
        if self.condition_branching_expected.is_some() {
            conditional_branching = self.condition_branching_expected.unwrap() != ctx.pc;
        }
        let assert = conditional_branching || self.jump;
        self.condition_branching_expected = None;
        self.jump = false;
        assert
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
    fn test_pipelining_simple() {
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
