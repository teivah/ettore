use crate::opcodes::*;
use queues::*;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::sync::Once;

const CYCLES_L1_ACCESS: f32 = 1.;
const CYCLES_MEMORY_ACCESS: f32 = 50. + CYCLES_L1_ACCESS;
const L1I_CACHE_LINE: i32 = 64 * 8;

pub struct Mvm3<'a> {
    ctx: Context,

    fetch_unit: FetchUnit,
    decode_bus: Bus<usize>,
    decode_unit: DecodeUnit,
    execute_bus: Bus<&'a Box<dyn InstructionRunner>>,
    execute_unit: ExecuteUnit<'a>,
    write_bus: Bus<ExecutionContext>,
    write_unit: WriteUnit,
    branch_unit: BranchUnit,
}

pub struct Bus<T: Clone> {
    entry: Queue<Vec<T>>,
    buffer: Queue<Vec<T>>,
    queue: Queue<T>,
    max: usize,
}

impl<T: Clone> Bus<T> {
    fn new(max: usize) -> Self {
        Bus {
            entry: queue![],
            buffer: queue![],
            queue: queue![],
            max,
        }
    }

    fn flush(&mut self) {
        self.entry = queue![];
        self.buffer = queue![];
        self.queue = queue![];
    }

    fn add(&mut self, t: Vec<T>) {
        self.entry.add(t).unwrap();
    }

    fn get(&mut self) -> T {
        self.queue.remove().unwrap()
    }

    fn peek(&mut self) -> T {
        self.queue.peek().unwrap()
    }

    fn is_full(&self) -> bool {
        self.queue.size() == self.max || self.entry.size() == self.max
    }

    fn is_empty(&self) -> bool {
        self.queue.size() == 0 && self.buffer.size() == 0 && self.entry.size() == 0
    }

    fn contains_element_in_buffer(&self) -> bool {
        self.buffer.size() != 0
    }

    fn contains_element_in_queue(&self) -> bool {
        self.queue.size() != 0
    }

    fn contains_element_in_entry(&self) -> bool {
        self.entry.size() != 0
    }

    fn connect(&mut self) {
        if self.queue.size() == self.max {
            return;
        }

        while self.buffer.size() != 0 {
            let list = self.buffer.remove().unwrap();
            for elem in list {
                self.queue.add(elem).unwrap();
            }
        }
        self.buffer = queue![];

        while self.entry.size() != 0 {
            let list = self.entry.remove().unwrap();
            self.buffer.add(list).unwrap();
        }
        self.entry = queue![];
    }
}

impl<T: Clone> fmt::Display for Bus<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "entry={},queue={},buffer={}",
            self.entry.size(),
            self.queue.size(),
            self.buffer.size()
        )
    }
}

impl<'a> Mvm3<'a> {
    pub fn run(&mut self, application: &'a Application) -> Result<f32, String> {
        let mut cycles: f32 = 0.;
        loop {
            cycles += 1.;
            self.log(cycles);

            // Fetch
            self.fetch_unit.cycle(application, &mut self.decode_bus);

            // Decode
            self.decode_bus.connect();
            self.decode_unit
                .cycle(application, &mut self.decode_bus, &mut self.execute_bus);

            // Execute
            self.execute_bus.connect();

            // Create branch unit assertions
            self.branch_unit
                .assert(&mut self.ctx, &mut self.execute_bus);

            // Execute
            self.execute_unit.cycle(
                &mut self.ctx,
                application,
                &mut self.execute_bus,
                &mut self.write_bus,
            )?;

            // Branch unit assertions check
            let mut flush = false;
            if self
                .branch_unit
                .pipeline_to_be_flushed(&self.ctx, &self.write_bus)
            {
                flush = true;
            }

            // Write back
            self.write_bus.connect();
            self.write_unit.cycle(&mut self.ctx, &mut self.write_bus);

            if flush {
                if self.write_bus.contains_element_in_buffer() {
                    // We need to waste a cycle to write the element in the queue buffer
                    cycles += 1.;
                    self.write_bus.connect();
                    self.write_unit.cycle(&mut self.ctx, &mut self.write_bus);
                }

                self.flush(self.ctx.pc);
            }
            if self.is_complete() {
                break;
            }
        }
        return Ok(cycles);
    }

    fn flush(&mut self, pc: i32) {
        self.fetch_unit.flush(pc);
        self.decode_unit.flush();
        self.decode_bus.flush();
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

    fn log(&self, cycles: f32) {
        log::debug!("cycles={}", cycles);
        log::debug!(
            "t0={},t1={},t2={},t3={}",
            self.ctx.registers[RegisterType::T0],
            self.ctx.registers[RegisterType::T1],
            self.ctx.registers[RegisterType::T2],
            self.ctx.registers[RegisterType::T3]
        );
        log::debug!(
            "decode: {}, execute: {}, write: {}",
            self.decode_bus,
            self.execute_bus,
            self.write_bus
        );
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
        self.boundary = (pc, pc + L1I_CACHE_LINE);
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
            out_bus.add(vec![(current_pc / 4) as usize]);
        }
    }

    fn flush(&mut self, pc: i32) {
        self.processing = false;
        self.complete = false;
        self.pc = pc;
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
        if !in_bus.contains_element_in_queue() || out_bus.is_full() {
            return;
        }
        let idx = in_bus.get();
        let runner = &application.instructions[idx];
        out_bus.add(vec![runner]);
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

#[derive(Clone)]
struct ExecutionContext {
    execution: Execution,
    instruction_type: InstructionType,
    write_registers: Vec<RegisterType>,
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
        out_bus: &mut Bus<ExecutionContext>,
    ) -> Result<(), String> {
        if !self.processing {
            if !in_bus.contains_element_in_queue() {
                return Ok(());
            }

            let runner = in_bus.get();
            self.runner = Some(runner);
            self.remaining_cycles = cycles_per_instruction(runner.instruction_type());
            self.processing = true;
        }

        self.remaining_cycles -= 1.;
        if self.remaining_cycles != 0. {
            return Ok(());
        }

        if out_bus.is_full() {
            self.remaining_cycles = 1.;
            return Ok(());
        }

        let runner = self.runner.unwrap();

        // To avoid writeback hazard, if the pipeline contains read registers not written yet, we wait for it.
        if ctx.contain_written_registers(&runner.read_registers()) {
            self.remaining_cycles = 1.;
            return Ok(());
        }

        log::debug!(
            "execute {:?} {:?} {:?}",
            runner.instruction_type(),
            runner.write_registers(),
            ctx.read_registers,
        );

        let execution = runner.run(ctx, &application.labels)?;
        ctx.pc = execution.pc;
        out_bus.add(vec![ExecutionContext {
            execution,
            instruction_type: runner.instruction_type(),
            write_registers: runner.write_registers(),
        }]);
        ctx.add_write_registers(runner.write_registers());
        self.runner = None;
        self.processing = false;
        return Ok(());
    }

    fn is_empty(&self) -> bool {
        !self.processing
    }
}

struct WriteUnit {}

impl WriteUnit {
    fn new() -> Self {
        WriteUnit {}
    }

    fn cycle(&mut self, ctx: &mut Context, write_bus: &mut Bus<ExecutionContext>) {
        if !write_bus.contains_element_in_queue() {
            return;
        }

        let execution = write_bus.get();
        if write_back(&execution.instruction_type) {
            log::debug!(
                "write value {} to register {:?}",
                execution.execution.value,
                execution.execution.register
            );
            ctx.write(&execution.execution);
            ctx.delete_write_registers(&execution.write_registers)
        }
    }

    fn is_empty(&self) -> bool {
        true
    }
}

struct BranchUnit {
    condition_branching_expected: Option<i32>,
    jump: bool,
}

impl<'a> BranchUnit {
    fn new() -> Self {
        BranchUnit {
            condition_branching_expected: None,
            jump: false,
        }
    }

    fn assert(&mut self, ctx: &mut Context, execute_bus: &mut Bus<&'a Box<dyn InstructionRunner>>) {
        if execute_bus.contains_element_in_queue() {
            let runner = execute_bus.peek();
            let instruction_type = runner.instruction_type();
            if jump(&instruction_type) {
                self.jump();
            } else if conditional_branching(&instruction_type) {
                self.conditional_branching(ctx.pc + 4)
            }
        }
    }

    fn jump(&mut self) {
        self.jump = true;
    }

    fn conditional_branching(&mut self, expected: i32) {
        self.condition_branching_expected = Some(expected);
    }

    fn pipeline_to_be_flushed(&mut self, ctx: &Context, write_bus: &Bus<ExecutionContext>) -> bool {
        if !write_bus.contains_element_in_entry() {
            return false;
        }

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

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    fn assert(
        init_registers: HashMap<RegisterType, i32>,
        memory_bytes: usize,
        init_memory: HashMap<usize, i8>,
        instructions: &str,
        assertions_registers: HashMap<RegisterType, i32>,
        assertions_memory: HashMap<usize, i8>,
        expected_cycles: f32,
    ) {
        let application = parse(instructions.to_string()).unwrap();
        let mut runner = Mvm3::new(memory_bytes);
        for register in init_registers {
            runner.ctx.registers[register.0] = register.1;
        }
        for memory in init_memory {
            runner.ctx.memory[memory.0] = memory.1;
        }
        let cycles = runner.run(&application).unwrap();
        assert_eq!(expected_cycles, cycles);
        for assertion in assertions_registers {
            assert_eq!(assertion.1, runner.ctx.registers[assertion.0]);
        }
        for assertion in assertions_memory {
            assert_eq!(assertion.1, runner.ctx.memory[assertion.0]);
        }
    }
    //
    // #[test]
    // fn test_prime_number() {
    //     let bits = bytes_from_low_bits(1109);
    //     assert(
    //         HashMap::new(),
    //         5,
    //         map! {0 => bits.0,1 => bits.1,2 => bits.2,3 => bits.3},
    //         fs::read_to_string("res/risc/prime-number.asm")
    //             .unwrap()
    //             .as_str()
    //             .borrow(),
    //         map! {RegisterType::A0 => 4},
    //         map! {4=>1},
    //         5149.,
    //     );
    // }
    //
    // #[test]
    // fn test_prime_number_1109() {
    //     assert(
    //         HashMap::new(),
    //         5,
    //         HashMap::new(),
    //         fs::read_to_string("res/risc/prime-number-1109.asm")
    //             .unwrap()
    //             .as_str()
    //             .borrow(),
    //         map! {RegisterType::A0 => 4},
    //         map! {4=>1},
    //         5200.,
    //     );
    // }

    #[test]
    fn test_pipelining_simple() {
        setup();
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "addi t0, zero, 1",
            map! {RegisterType::T0=> 1},
            HashMap::new(),
            54.,
        );
    }

    #[test]
    fn test_pipelining_multiple() {
        setup();
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "addi t0, zero, 1
            addi t1, zero, 2
            addi t2, zero, 3",
            map! {RegisterType::T0=> 1, RegisterType::T1 => 2, RegisterType::T2 => 3},
            HashMap::new(),
            56.,
        );
    }

    #[test]
    fn test_pipelining_jal() {
        setup();
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "addi t0, zero, 1
            jal t2, foo
            addi t1, zero, 2
            foo:
            addi t2, zero, 3",
            map! {RegisterType::T0=> 1, RegisterType::T1 => 0, RegisterType::T2 => 3 },
            HashMap::new(),
            59.,
        );
    }

    #[test]
    fn test_pipelining_conditional_branching_true() {
        setup();
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "addi t0, zero, 1
            addi t1, zero, 1
            beq t0, t1, foo
            addi t1, zero, 2
            foo:
            addi t2, zero, 3",
            map! {RegisterType::T0=> 1, RegisterType::T1 => 1, RegisterType::T2 => 3 },
            HashMap::new(),
            61.,
        );
    }

    #[test]
    fn test_pipelining_conditional_branching_false() {
        setup();
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "addi t0, zero, 0
            addi t1, zero, 1
            beq t0, t1, foo
            addi t1, zero, 2
            foo:
            addi t2, zero, 3",
            map! {RegisterType::T0=> 0, RegisterType::T1 => 2, RegisterType::T2 => 3 },
            HashMap::new(),
            59.,
        );
    }
}
