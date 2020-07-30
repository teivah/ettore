use crate::bit::*;
use enum_map::{Enum, EnumMap};
use std::collections::HashMap;

pub struct Application {
    pub instructions: Vec<Box<dyn InstructionRunner>>,
    pub labels: HashMap<String, i32>,
}

struct Runner {
    ctx: Context,
    application: Application,
}

impl Runner {
    fn new(application: Application, memory_bytes: usize) -> Self {
        Runner {
            ctx: Context::new(memory_bytes),
            application,
        }
    }

    fn run(&mut self) -> Result<(), String> {
        while self.ctx.pc / 4 < self.application.instructions.len() as i32 {
            let runner = &self.application.instructions[(self.ctx.pc / 4) as usize];
            runner.run(&mut self.ctx, &self.application.labels)?;
        }
        return Ok(());
    }
}

pub struct Context {
    registers: EnumMap<RegisterType, i32>,
    memory: Vec<i8>,
    pc: i32,
}

impl Context {
    fn new(memory_bytes: usize) -> Self {
        Context {
            registers: EnumMap::<RegisterType, i32>::new(),
            memory: vec![0; memory_bytes],
            pc: 0,
        }
    }
}

pub trait InstructionRunner {
    fn run(&self, ctx: &mut Context, labels: &HashMap<String, i32>) -> Result<(), String>;
}

#[derive(PartialEq, Debug)]
pub struct Add {
    pub rd: RegisterType,
    pub rs1: RegisterType,
    pub rs2: RegisterType,
}

impl InstructionRunner for Add {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs1] + ctx.registers[self.rs2];
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Addi {
    pub imm: i32,
    pub rd: RegisterType,
    pub rs: RegisterType,
}

impl InstructionRunner for Addi {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs] + self.imm;
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct And {
    pub rd: RegisterType,
    pub rs1: RegisterType,
    pub rs2: RegisterType,
}

impl InstructionRunner for And {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs1] & ctx.registers[self.rs2];
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Andi {
    pub imm: i32,
    pub rd: RegisterType,
    pub rs: RegisterType,
}

impl InstructionRunner for Andi {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs] & self.imm;
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Auipc {
    pub rd: RegisterType,
    pub imm: i32,
}

impl InstructionRunner for Auipc {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.pc + (self.imm << 12);
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Beq {
    pub rs1: RegisterType,
    pub rs2: RegisterType,
    pub label: String,
}

impl InstructionRunner for Beq {
    fn run(&self, ctx: &mut Context, labels: &HashMap<String, i32>) -> Result<(), String> {
        if ctx.registers[self.rs1] == ctx.registers[self.rs2] {
            let addr: i32;
            match labels.get(self.label.as_str()) {
                Some(v) => addr = *v,
                None => return Err(format_args!("label {} does not exist", self.label).to_string()),
            }
            ctx.pc = addr;
        } else {
            ctx.pc += 4;
        }
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Bge {
    pub rs1: RegisterType,
    pub rs2: RegisterType,
    pub label: String,
}

impl InstructionRunner for Bge {
    fn run(&self, ctx: &mut Context, labels: &HashMap<String, i32>) -> Result<(), String> {
        if ctx.registers[self.rs1] >= ctx.registers[self.rs2] {
            let addr: i32;
            match labels.get(self.label.as_str()) {
                Some(v) => addr = *v,
                None => return Err(format_args!("label {} does not exist", self.label).to_string()),
            }
            ctx.pc = addr;
        } else {
            ctx.pc += 4;
        }
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Bgeu {
    pub rs1: RegisterType,
    pub rs2: RegisterType,
    pub label: String,
}

impl InstructionRunner for Bgeu {
    fn run(&self, ctx: &mut Context, labels: &HashMap<String, i32>) -> Result<(), String> {
        if ctx.registers[self.rs1] >= ctx.registers[self.rs2] {
            let addr: i32;
            match labels.get(self.label.as_str()) {
                Some(v) => addr = *v,
                None => return Err(format_args!("label {} does not exist", self.label).to_string()),
            }
            ctx.pc = addr;
        } else {
            ctx.pc += 4;
        }
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Blt {
    pub rs1: RegisterType,
    pub rs2: RegisterType,
    pub label: String,
}

impl InstructionRunner for Blt {
    fn run(&self, ctx: &mut Context, labels: &HashMap<String, i32>) -> Result<(), String> {
        if ctx.registers[self.rs1] < ctx.registers[self.rs2] {
            let addr: i32;
            match labels.get(self.label.as_str()) {
                Some(v) => addr = *v,
                None => return Err(format_args!("label {} does not exist", self.label).to_string()),
            }
            ctx.pc = addr;
        } else {
            ctx.pc += 4;
        }
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Bne {
    pub rs1: RegisterType,
    pub rs2: RegisterType,
    pub label: String,
}

impl InstructionRunner for Bne {
    fn run(&self, ctx: &mut Context, labels: &HashMap<String, i32>) -> Result<(), String> {
        if ctx.registers[self.rs1] != ctx.registers[self.rs2] {
            let addr: i32;
            match labels.get(self.label.as_str()) {
                Some(v) => addr = *v,
                None => return Err(format_args!("label {} does not exist", self.label).to_string()),
            }
            ctx.pc = addr;
        } else {
            ctx.pc += 4;
        }
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Jal {
    pub label: String,
    pub rd: RegisterType,
}

impl InstructionRunner for Jal {
    fn run(&self, ctx: &mut Context, labels: &HashMap<String, i32>) -> Result<(), String> {
        let addr: i32;
        match labels.get(self.label.as_str()) {
            Some(v) => addr = *v,
            None => return Err(format_args!("label {} does not exist", self.label).to_string()),
        }

        ctx.registers[self.rd] = ctx.pc + 4;
        ctx.pc = addr;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Jalr {
    pub rd: RegisterType,
    pub rs: RegisterType,
    pub imm: i32,
}

impl InstructionRunner for Jalr {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.pc + 4;
        ctx.pc = ctx.registers[self.rs] + self.imm;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Lui {
    pub rd: RegisterType,
    pub imm: i32,
}

impl InstructionRunner for Lui {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = self.imm << 12;
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Lb {
    pub rs2: RegisterType,
    pub offset: i32,
    pub rs1: RegisterType,
}

impl InstructionRunner for Lb {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        let idx = ctx.registers[self.rs1] + self.offset;
        let n = ctx.memory[idx as usize];
        ctx.registers[self.rs2] = n as i32;

        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Lh {
    pub rs2: RegisterType,
    pub offset: i32,
    pub rs1: RegisterType,
}

impl InstructionRunner for Lh {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        let mut idx = ctx.registers[self.rs1] + self.offset;
        let i1 = ctx.memory[idx as usize];
        idx += 1;
        let i2 = ctx.memory[idx as usize];

        let n = i32_from_bytes(i1, i2, 0, 0);
        ctx.registers[self.rs2] = n;

        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Lw {
    pub rs2: RegisterType,
    pub offset: i32,
    pub rs1: RegisterType,
}

impl InstructionRunner for Lw {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        let mut idx = ctx.registers[self.rs1] + self.offset;
        let i1 = ctx.memory[idx as usize];
        idx += 1;
        let i2 = ctx.memory[idx as usize];
        idx += 1;
        let i3 = ctx.memory[idx as usize];
        idx += 1;
        let i4 = ctx.memory[idx as usize];

        let n = i32_from_bytes(i1, i2, i3, i4);
        ctx.registers[self.rs2] = n;

        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Nop {}

impl InstructionRunner for Nop {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Or {
    pub rd: RegisterType,
    pub rs1: RegisterType,
    pub rs2: RegisterType,
}

impl InstructionRunner for Or {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs1] | ctx.registers[self.rs2];
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Ori {
    pub imm: i32,
    pub rd: RegisterType,
    pub rs: RegisterType,
}

impl InstructionRunner for Ori {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs] | self.imm;
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Sb {
    pub rs2: RegisterType,
    pub offset: i32,
    pub rs1: RegisterType,
}

impl InstructionRunner for Sb {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        let idx = ctx.registers[self.rs1] + self.offset;
        let n = ctx.registers[self.rs2];
        ctx.memory[idx as usize] = n as i8;
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Sh {
    pub rs2: RegisterType,
    pub offset: i32,
    pub rs1: RegisterType,
}

impl InstructionRunner for Sh {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        let mut idx = ctx.registers[self.rs1] + self.offset;
        let n = ctx.registers[self.rs2];
        let bytes = bytes_from_low_bits(n);
        ctx.memory[idx as usize] = bytes.0;
        idx += 1;
        ctx.memory[idx as usize] = bytes.1;
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Sll {
    pub rd: RegisterType,
    pub rs1: RegisterType,
    pub rs2: RegisterType,
}

impl InstructionRunner for Sll {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs1] << ctx.registers[self.rs2];
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Slli {
    pub rd: RegisterType,
    pub rs: RegisterType,
    pub imm: i32,
}

impl InstructionRunner for Slli {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs] << self.imm;
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Slt {
    pub rd: RegisterType,
    pub rs1: RegisterType,
    pub rs2: RegisterType,
}

impl InstructionRunner for Slt {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        if ctx.registers[self.rs1] < ctx.registers[self.rs2] {
            ctx.registers[self.rd] = 1
        } else {
            ctx.registers[self.rd] = 0
        }
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Slti {
    pub rd: RegisterType,
    pub rs: RegisterType,
    pub imm: i32,
}

impl InstructionRunner for Slti {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        if ctx.registers[self.rs] < self.imm {
            ctx.registers[self.rd] = 1
        } else {
            ctx.registers[self.rd] = 0
        }
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Sra {
    pub rd: RegisterType,
    pub rs1: RegisterType,
    pub rs2: RegisterType,
}

impl InstructionRunner for Sra {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs1] >> ctx.registers[self.rs2];
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Srai {
    pub rd: RegisterType,
    pub rs: RegisterType,
    pub imm: i32,
}

impl InstructionRunner for Srai {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs] >> self.imm;
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Srl {
    pub rd: RegisterType,
    pub rs1: RegisterType,
    pub rs2: RegisterType,
}

impl InstructionRunner for Srl {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs1] >> ctx.registers[self.rs2];
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Srli {
    pub rd: RegisterType,
    pub rs: RegisterType,
    pub imm: i32,
}

impl InstructionRunner for Srli {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs] >> self.imm;
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Sub {
    pub rd: RegisterType,
    pub rs1: RegisterType,
    pub rs2: RegisterType,
}

impl InstructionRunner for Sub {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs1] - ctx.registers[self.rs2];
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Sw {
    pub rs2: RegisterType,
    pub offset: i32,
    pub rs1: RegisterType,
}

impl InstructionRunner for Sw {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        let mut idx = ctx.registers[self.rs1] + self.offset;
        let n = ctx.registers[self.rs2];
        let bytes = bytes_from_low_bits(n);
        ctx.memory[idx as usize] = bytes.0;
        idx += 1;
        ctx.memory[idx as usize] = bytes.1;
        idx += 1;
        ctx.memory[idx as usize] = bytes.2;
        idx += 1;
        ctx.memory[idx as usize] = bytes.3;
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Xor {
    pub rd: RegisterType,
    pub rs1: RegisterType,
    pub rs2: RegisterType,
}

impl InstructionRunner for Xor {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs1] ^ ctx.registers[self.rs2];
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug)]
pub struct Xori {
    pub imm: i32,
    pub rd: RegisterType,
    pub rs: RegisterType,
}

impl InstructionRunner for Xori {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.registers[self.rs] ^ self.imm;
        ctx.pc += 4;
        return Ok(());
    }
}

#[derive(PartialEq, Debug, Enum, Clone, Copy, Eq, Hash)]
pub enum RegisterType {
    ZERO,
    RA,
    SP,
    GP,
    TP,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    macro_rules! map(
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
    fn test_add() {
        assert(
            map! {RegisterType::T1 => 1, RegisterType::T2 => 2},
            0,
            HashMap::new(),
            "add t0, t1, t2",
            map! {RegisterType::T0 => 3},
            HashMap::new(),
        );
    }

    #[test]
    fn test_addi() {
        assert(
            map! {RegisterType::T1 => 1},
            0,
            HashMap::new(),
            "addi t0, t1, 1",
            map! {RegisterType::T0 => 2},
            HashMap::new(),
        );
    }

    #[test]
    fn test_and() {
        assert(
            map! {RegisterType::T1 => 1, RegisterType::T2 => 3},
            0,
            HashMap::new(),
            "and t0, t1, t2",
            map! {RegisterType::T0 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_andi() {
        assert(
            map! {RegisterType::T1 => 1},
            0,
            HashMap::new(),
            "andi t0, t1, 3",
            map! {RegisterType::T0 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_auipc() {
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "auipc t0, 0
            auipc t0, 0
            auipc t0, 0",
            map! {RegisterType::T0 => 8},
            HashMap::new(),
        );

        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "auipc t0, 1
            auipc t0, 1
            auipc t0, 1",
            map! {RegisterType::T0 => 4104},
            HashMap::new(),
        );
    }

    #[test]
    fn test_beq() {
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "beq t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 0, RegisterType::T1 => 1},
            HashMap::new(),
        );

        assert(
            map! {RegisterType::T0 => 1},
            0,
            HashMap::new(),
            "beq t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 2, RegisterType::T1 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_bge() {
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "bge t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 0, RegisterType::T1 => 1},
            HashMap::new(),
        );

        assert(
            map! {RegisterType::T1 => 10},
            0,
            HashMap::new(),
            "bge t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 2, RegisterType::T1 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_bgeu() {
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "bgeu t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 0, RegisterType::T1 => 1},
            HashMap::new(),
        );

        assert(
            map! {RegisterType::T1 => 10},
            0,
            HashMap::new(),
            "bgeu t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 2, RegisterType::T1 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_blt() {
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "blt t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 2, RegisterType::T1 => 1},
            HashMap::new(),
        );

        assert(
            map! {RegisterType::T1 => 10},
            0,
            HashMap::new(),
            "blt t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 0, RegisterType::T1 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_bltu() {
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "blt t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 2, RegisterType::T1 => 1},
            HashMap::new(),
        );

        assert(
            map! {RegisterType::T1 => 10},
            0,
            HashMap::new(),
            "blt t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 0, RegisterType::T1 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_bne() {
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "bne t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 2, RegisterType::T1 => 1},
            HashMap::new(),
        );

        assert(
            map! {RegisterType::T0 => 1},
            0,
            HashMap::new(),
            "bne t0, t1, foo
            addi t0, zero, 2
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 1, RegisterType::T1 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_jal() {
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "jal t0, foo
            addi t1, zero, 1
            foo:            
            addi t2, zero, 2",
            map! {RegisterType::T0 => 4, RegisterType::T1 => 0,RegisterType::T2 => 2},
            HashMap::new(),
        );
    }

    #[test]
    fn test_jalr() {
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "addi t1, zero, 4
            jalr t0, t1, 8
            foo:            
            addi t2, zero, 2
            addi t1, zero, 2",
            map! {RegisterType::T0 => 8, RegisterType::T1 => 2,RegisterType::T2 => 0},
            HashMap::new(),
        );
    }

    #[test]
    fn test_lui() {
        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "lui t0, 0",
            map! {RegisterType::T0 => 0},
            HashMap::new(),
        );

        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "lui t0, 1",
            map! {RegisterType::T0 => 4096},
            HashMap::new(),
        );

        assert(
            HashMap::new(),
            0,
            HashMap::new(),
            "lui t0, 3",
            map! {RegisterType::T0 => 12288},
            HashMap::new(),
        );
    }

    #[test]
    fn test_or() {
        assert(
            map! {RegisterType::T1 => 1, RegisterType::T2 => 2},
            0,
            HashMap::new(),
            "or t0, t1, t2",
            map! {RegisterType::T0 => 3},
            HashMap::new(),
        );
    }

    #[test]
    fn test_ori() {
        assert(
            map! {RegisterType::T1 => 1},
            0,
            HashMap::new(),
            "ori t0, t1, 2",
            map! {RegisterType::T0 => 3},
            HashMap::new(),
        );
    }

    #[test]
    fn test_sll() {
        assert(
            map! {RegisterType::T1 => 1,RegisterType::T2 => 2},
            0,
            HashMap::new(),
            "sll t0, t1, t2",
            map! {RegisterType::T0 => 4},
            HashMap::new(),
        );
    }

    #[test]
    fn test_slli() {
        assert(
            map! {RegisterType::T1 => 1},
            0,
            HashMap::new(),
            "slli t0, t1, 2",
            map! {RegisterType::T0 => 4},
            HashMap::new(),
        );
    }

    #[test]
    fn test_slt() {
        assert(
            map! {RegisterType::T1 => 2,RegisterType::T2 => 3},
            0,
            HashMap::new(),
            "slt t0, t1, t2",
            map! {RegisterType::T0 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_slti() {
        assert(
            map! {RegisterType::T1 => 2},
            0,
            HashMap::new(),
            "slti t0, t1, 5",
            map! {RegisterType::T0 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_sltu() {
        assert(
            map! {RegisterType::T1 => 2,RegisterType::T2 => 3},
            0,
            HashMap::new(),
            "sltu t0, t1, t2",
            map! {RegisterType::T0 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_sra() {
        assert(
            map! {RegisterType::T1 => 2, RegisterType::T2 => 1},
            0,
            HashMap::new(),
            "sra t0, t1, t2",
            map! {RegisterType::T0 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_srai() {
        assert(
            map! {RegisterType::T1 => 2},
            0,
            HashMap::new(),
            "srai t0, t1, 1",
            map! {RegisterType::T0 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_srl() {
        assert(
            map! {RegisterType::T1 => 4,RegisterType::T2 => 2},
            0,
            HashMap::new(),
            "srl t0, t1, t2",
            map! {RegisterType::T0 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_srli() {
        assert(
            map! {RegisterType::T1 => 4},
            0,
            HashMap::new(),
            "srli t0, t1, 2",
            map! {RegisterType::T0 => 1},
            HashMap::new(),
        );
    }

    #[test]
    fn test_sub() {
        assert(
            map! {RegisterType::T1 => 10, RegisterType::T2 => 6},
            0,
            HashMap::new(),
            "sub t0, t1, t2",
            map! {RegisterType::T0 => 4},
            HashMap::new(),
        );
    }

    #[test]
    fn test_sb_lb() {
        assert(
            map! {RegisterType::T0 => 16, RegisterType::T1 => 2},
            8,
            HashMap::new(),
            "sb t0, 2, t1
            lb t2, 2, t1",
            map! {RegisterType::T2 => 16},
            map! { 4=>16},
        );

        assert(
            map! {RegisterType::T0 => 2047, RegisterType::T1 => 2},
            8,
            HashMap::new(),
            "sb t0, 2, t1
            lb t2, 2, t1",
            map! {RegisterType::T2 => -1},
            map! { 4=>-1},
        );
    }

    #[test]
    fn test_sh_lh() {
        assert(
            map! {RegisterType::T0 => 64, RegisterType::T1 => 2},
            8,
            map! { 4=>1, 5=>1},
            "sh t0, 2, t1
            lh t2, 2, t1",
            map! {RegisterType::T2 => 64},
            map! { 4=>64, 5=>0},
        );

        assert(
            map! {RegisterType::T0 => 2047, RegisterType::T1 => 2},
            8,
            map! { 4=>1, 5=>1},
            "sh t0, 2, t1
            lh t2, 2, t1",
            map! {RegisterType::T2 => 2047},
            map! { 4=>-1, 5=>7},
        );
    }

    #[test]
    fn test_sw_lw() {
        assert(
            map! {RegisterType::T0 => 258, RegisterType::T1 => 2},
            8,
            map! {4=>1, 5=>1, 6=>1, 7=>1 },
            "sw t0, 2, t1
            lw t2, 2, t1",
            map! {RegisterType::T2 => 258},
            map! {4=>2, 5=>1, 6=>0, 7=>0 },
        );

        assert(
            map! {RegisterType::T0 => 2047, RegisterType::T1 => 2},
            8,
            map! { 4=>1, 5=>1, 6=>1, 7=>1 },
            "sw t0, 2, t1
            lw t2, 2, t1",
            map! {RegisterType::T2 => 2047},
            map! { 4=>-1, 5=>7, 6=>0, 7=>0 },
        );
    }

    #[test]
    fn test_xor() {
        assert(
            map! {RegisterType::T1 => 3, RegisterType::T2 => 4},
            0,
            HashMap::new(),
            "xor t0, t1, t2",
            map! {RegisterType::T0 => 7},
            HashMap::new(),
        );
    }

    #[test]
    fn test_xori() {
        assert(
            map! {RegisterType::T1 => 3},
            0,
            HashMap::new(),
            "xori t0, t1, 4",
            map! {RegisterType::T0 => 7},
            HashMap::new(),
        );
    }
}
