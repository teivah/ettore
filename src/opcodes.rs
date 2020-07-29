use either::*;
use enum_map::{Enum, EnumMap};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

struct Runner {
    ctx: Context,
    instructions: Vec<Box<dyn InstructionRunner>>,
    labels: HashMap<String, i32>,
}

impl Runner {
    fn new(instructions: Vec<Box<dyn InstructionRunner>>, labels: HashMap<String, i32>) -> Self {
        Runner {
            ctx: Context::new(),
            instructions,
            labels,
        }
    }

    fn run(&mut self) -> Result<(), String> {
        while self.ctx.pc / 4 < self.instructions.len() as i32 {
            let runner = &self.instructions[(self.ctx.pc / 4) as usize];
            runner.run(&mut self.ctx, &self.labels)?;
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
    pub imm: i32,
    pub rd: RegisterType,
}

impl InstructionRunner for Auipc {
    fn run(&self, ctx: &mut Context, _: &HashMap<String, i32>) -> Result<(), String> {
        ctx.registers[self.rd] = ctx.pc + (self.imm << 12);
        ctx.pc += 4;
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
pub struct Lui {
    pub imm: i32,
    pub rd: RegisterType,
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
    pub imm: i32,
    pub rd: RegisterType,
    pub rs: RegisterType,
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

#[derive(PartialEq, Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub i1: Either<RegisterType, String>,
    pub i2: Either<RegisterType, String>,
    pub i3: Either<RegisterType, String>,
}

#[derive(PartialEq, Debug, Enum, Clone, Copy)]
pub enum InstructionType {
    ADD,
    ADDI,
    AND,
    ANDI,
    AUIPC,
    JAL,
    LUI,
    NOP,
    OR,
    ORI,
    SLL,
    SLLI,
    SLT,
    SLTI,
    SLTU,
    SRA,
    SRAI,
    SRL,
    SRLI,
    SUB,
    XOR,
    XORI,
}

#[derive(PartialEq, Debug, Enum, Clone, Copy)]
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

    #[test]
    fn test_runner() {
        let instructions: Vec<Box<InstructionRunner>> = vec![
            Box::new(Add {
                rd: RegisterType::T0,
                rs1: RegisterType::T1,
                rs2: RegisterType::T2,
            }),
            Box::new(Add {
                rd: RegisterType::T0,
                rs1: RegisterType::T0,
                rs2: RegisterType::T2,
            }),
        ];
        let mut runner = Runner::new(instructions, HashMap::new());
        runner.ctx.registers[RegisterType::T1] = 1;
        runner.ctx.registers[RegisterType::T2] = 2;
        runner.run().unwrap();
        assert_eq!(runner.ctx.registers[RegisterType::T0], 5);
    }

    #[test]
    fn test_add() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;
        ctx.registers[RegisterType::T2] = 2;

        let runner = Add {
            rd: RegisterType::T0,
            rs1: RegisterType::T1,
            rs2: RegisterType::T2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 3);
    }

    #[test]
    fn test_addi() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;

        let runner = Addi {
            rd: RegisterType::T0,
            rs: RegisterType::T1,
            imm: 1,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 2);
    }

    #[test]
    fn test_and() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;
        ctx.registers[RegisterType::T2] = 3;

        let runner = And {
            rd: RegisterType::T0,
            rs1: RegisterType::T1,
            rs2: RegisterType::T2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_andi() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;

        let runner = Andi {
            rd: RegisterType::T0,
            rs: RegisterType::T1,
            imm: 3,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_auipc() {
        let mut instructions: Vec<Box<InstructionRunner>> = vec![
            Box::new(Auipc {
                rd: RegisterType::T0,
                imm: 0,
            }),
            Box::new(Auipc {
                rd: RegisterType::T0,
                imm: 0,
            }),
            Box::new(Auipc {
                rd: RegisterType::T0,
                imm: 0,
            }),
        ];
        let mut runner = Runner::new(instructions, HashMap::new());
        runner.run().unwrap();
        assert_eq!(runner.ctx.registers[RegisterType::T0], 8);

        instructions = vec![
            Box::new(Auipc {
                rd: RegisterType::T0,
                imm: 1,
            }),
            Box::new(Auipc {
                rd: RegisterType::T0,
                imm: 1,
            }),
            Box::new(Auipc {
                rd: RegisterType::T0,
                imm: 1,
            }),
        ];
        let mut runner = Runner::new(instructions, HashMap::new());
        runner.run().unwrap();
        assert_eq!(runner.ctx.registers[RegisterType::T0], 4104);
    }

    #[test]
    fn test_jal() {
        let instructions: Vec<Box<InstructionRunner>> = vec![
            Box::new(Jal {
                rd: RegisterType::T0,
                label: "foo".to_string(),
            }),
            Box::new(Addi {
                rd: RegisterType::T1,
                rs: RegisterType::ZERO,
                imm: 1,
            }),
            Box::new(Addi {
                rd: RegisterType::T2,
                rs: RegisterType::ZERO,
                imm: 2,
            }),
        ];
        let mut labels = HashMap::new();
        labels.insert("foo".to_string(), 8);

        let mut runner = Runner::new(instructions, labels);
        runner.run().unwrap();
        assert_eq!(runner.ctx.registers[RegisterType::T0], 4);
        assert_eq!(runner.ctx.registers[RegisterType::T1], 0);
        assert_eq!(runner.ctx.registers[RegisterType::T2], 2);
    }

    #[test]
    fn test_lui() {
        let mut ctx = Context::new();

        let runner = Lui {
            rd: RegisterType::T0,
            imm: 0,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 0);

        let runner = Lui {
            rd: RegisterType::T0,
            imm: 1,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 4096);

        let runner = Lui {
            rd: RegisterType::T0,
            imm: 3,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 12288);
    }

    #[test]
    fn test_or() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;
        ctx.registers[RegisterType::T2] = 2;

        let runner = Or {
            rd: RegisterType::T0,
            rs1: RegisterType::T1,
            rs2: RegisterType::T2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 3);
    }

    #[test]
    fn test_ori() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;

        let runner = Ori {
            rd: RegisterType::T0,
            rs: RegisterType::T1,
            imm: 2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 3);
    }

    #[test]
    fn test_sll() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;
        ctx.registers[RegisterType::T2] = 2;

        let runner = Sll {
            rd: RegisterType::T0,
            rs1: RegisterType::T1,
            rs2: RegisterType::T2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 4);
    }

    #[test]
    fn test_slli() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;

        let runner = Slli {
            rd: RegisterType::T0,
            rs: RegisterType::T1,
            imm: 2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 4);
    }

    #[test]
    fn test_slt() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 2;
        ctx.registers[RegisterType::T2] = 3;

        let runner = Slt {
            rd: RegisterType::T0,
            rs1: RegisterType::T1,
            rs2: RegisterType::T2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_slti() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 2;

        let runner = Slti {
            rd: RegisterType::T0,
            rs: RegisterType::T1,
            imm: 5,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_sra() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 2;
        ctx.registers[RegisterType::T2] = 1;

        let runner = Sra {
            rd: RegisterType::T0,
            rs1: RegisterType::T1,
            rs2: RegisterType::T2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_srai() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 2;

        let runner = Srai {
            rd: RegisterType::T0,
            rs: RegisterType::T1,
            imm: 1,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_srl() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 4;
        ctx.registers[RegisterType::T2] = 2;

        let runner = Srl {
            rd: RegisterType::T0,
            rs1: RegisterType::T1,
            rs2: RegisterType::T2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_srli() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 4;

        let runner = Srli {
            rd: RegisterType::T0,
            rs: RegisterType::T1,
            imm: 2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_sub() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 10;
        ctx.registers[RegisterType::T2] = 6;

        let runner = Sub {
            rd: RegisterType::T0,
            rs1: RegisterType::T1,
            rs2: RegisterType::T2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 4);
    }

    #[test]
    fn test_xor() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 3;
        ctx.registers[RegisterType::T2] = 4;

        let runner = Xor {
            rd: RegisterType::T0,
            rs1: RegisterType::T1,
            rs2: RegisterType::T2,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 7);
    }

    #[test]
    fn test_xori() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 3;

        let runner = Xori {
            rd: RegisterType::T0,
            rs: RegisterType::T1,
            imm: 4,
        };
        runner.run(&mut ctx, &HashMap::new()).unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 7);
    }
}
