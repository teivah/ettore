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
    fn new(application: Application) -> Self {
        Runner {
            ctx: Context::new(),
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
    pc: i32,
}

impl Context {
    fn new() -> Self {
        Context {
            registers: EnumMap::<RegisterType, i32>::new(),
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
pub struct Sltu {
    pub rd: RegisterType,
    pub rs1: RegisterType,
    pub rs2: RegisterType,
}

impl InstructionRunner for Sltu {
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
        registers: HashMap<RegisterType, i32>,
        instructions: &str,
        assertions: HashMap<RegisterType, i32>,
    ) {
        let application = parse(instructions.to_string()).unwrap();
        let mut runner = Runner::new(application);
        for register in registers {
            runner.ctx.registers[register.0] = register.1;
        }
        runner.run().unwrap();
        for assertion in assertions {
            assert_eq!(runner.ctx.registers[assertion.0], assertion.1);
        }
    }

    #[test]
    fn test_add() {
        assert(
            map! {RegisterType::T1 => 1, RegisterType::T2 => 2},
            "add t0, t1, t2",
            map! {RegisterType::T0 => 3},
        );
    }

    #[test]
    fn test_addi() {
        assert(
            map! {RegisterType::T1 => 1},
            "addi t0, t1, 1",
            map! {RegisterType::T0 => 2},
        );
    }

    #[test]
    fn test_and() {
        assert(
            map! {RegisterType::T1 => 1, RegisterType::T2 => 3},
            "and t0, t1, t2",
            map! {RegisterType::T0 => 1},
        );
    }

    #[test]
    fn test_andi() {
        assert(
            map! {RegisterType::T1 => 1},
            "andi t0, t1, 3",
            map! {RegisterType::T0 => 1},
        );
    }

    #[test]
    fn test_auipc() {
        assert(
            HashMap::new(),
            "auipc t0, 0
            auipc t0, 0
            auipc t0, 0",
            map! {RegisterType::T0 => 8},
        );

        assert(
            HashMap::new(),
            "auipc t0, 1
            auipc t0, 1
            auipc t0, 1",
            map! {RegisterType::T0 => 4104},
        );
    }

    #[test]
    fn test_beq() {
        assert(
            HashMap::new(),
            "beq t0, t1, foo
            addi t0, zero, 1
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 0, RegisterType::T1 => 1},
        );

        assert(
            map! {RegisterType::T0 => 1},
            "beq t0, t1, foo
            addi t0, zero, 1
            foo:
            addi t1, zero, 1",
            map! {RegisterType::T0 => 1, RegisterType::T1 => 1},
        );
    }

    #[test]
    fn test_jal() {
        assert(
            HashMap::new(),
            "jal t0, foo
            addi t1, zero, 1
            foo:            
            addi t2, zero, 2",
            map! {RegisterType::T0 => 4, RegisterType::T1 => 0,RegisterType::T2 => 2},
        );
    }

    #[test]
    fn test_jalr() {
        assert(
            HashMap::new(),
            "addi t1, zero, 4
            jalr t0, t1, 8
            foo:            
            addi t2, zero, 2
            addi t1, zero, 2",
            map! {RegisterType::T0 => 8, RegisterType::T1 => 2,RegisterType::T2 => 0},
        );
    }

    #[test]
    fn test_lui() {
        assert(HashMap::new(), "lui t0, 0", map! {RegisterType::T0 => 0});

        assert(HashMap::new(), "lui t0, 1", map! {RegisterType::T0 => 4096});

        assert(
            HashMap::new(),
            "lui t0, 3",
            map! {RegisterType::T0 => 12288},
        );
    }

    #[test]
    fn test_or() {
        assert(
            map! {RegisterType::T1 => 1, RegisterType::T2 => 2},
            "or t0, t1, t2",
            map! {RegisterType::T0 => 3},
        );
    }

    #[test]
    fn test_ori() {
        assert(
            map! {RegisterType::T1 => 1},
            "ori t0, t1, 2",
            map! {RegisterType::T0 => 3},
        );
    }

    #[test]
    fn test_sll() {
        assert(
            map! {RegisterType::T1 => 1,RegisterType::T2 => 2},
            "sll t0, t1, t2",
            map! {RegisterType::T0 => 4},
        );
    }

    #[test]
    fn test_slli() {
        assert(
            map! {RegisterType::T1 => 1},
            "slli t0, t1, 2",
            map! {RegisterType::T0 => 4},
        );
    }

    #[test]
    fn test_slt() {
        assert(
            map! {RegisterType::T1 => 2,RegisterType::T2 => 3},
            "slt t0, t1, t2",
            map! {RegisterType::T0 => 1},
        );
    }

    #[test]
    fn test_slti() {
        assert(
            map! {RegisterType::T1 => 2},
            "slti t0, t1, 5",
            map! {RegisterType::T0 => 1},
        );
    }

    #[test]
    fn test_sra() {
        assert(
            map! {RegisterType::T1 => 2, RegisterType::T2 => 1},
            "sra t0, t1, t2",
            map! {RegisterType::T0 => 1},
        );
    }

    #[test]
    fn test_srai() {
        assert(
            map! {RegisterType::T1 => 2},
            "srai t0, t1, 1",
            map! {RegisterType::T0 => 1},
        );
    }

    #[test]
    fn test_srl() {
        assert(
            map! {RegisterType::T1 => 4,RegisterType::T2 => 2},
            "srl t0, t1, t2",
            map! {RegisterType::T0 => 1},
        );
    }

    #[test]
    fn test_srli() {
        assert(
            map! {RegisterType::T1 => 4},
            "srli t0, t1, 2",
            map! {RegisterType::T0 => 1},
        );
    }

    #[test]
    fn test_sub() {
        assert(
            map! {RegisterType::T1 => 10, RegisterType::T2 => 6},
            "sub t0, t1, t2",
            map! {RegisterType::T0 => 4},
        );
    }

    #[test]
    fn test_xor() {
        assert(
            map! {RegisterType::T1 => 3, RegisterType::T2 => 4},
            "xor t0, t1, t2",
            map! {RegisterType::T0 => 7},
        );
    }

    #[test]
    fn test_xori() {
        assert(
            map! {RegisterType::T1 => 3},
            "xori t0, t1, 4",
            map! {RegisterType::T0 => 7},
        );
    }
}
