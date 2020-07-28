use either::*;
use enum_map::{enum_map, Enum, EnumMap};
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;

struct Runner {
    ctx: Context,
    instructions: Vec<Instruction>,
}

impl Runner {
    fn new(instructions: Vec<Instruction>) -> Self {
        Runner {
            ctx: Context::new(),
            instructions,
        }
    }

    fn run(&mut self) -> Result<(), String> {
        for instruction in &self.instructions {
            println!("before: {}", self.ctx.registers[RegisterType::T0]);
            let runner = Runner::get_runner(&instruction.instruction_type)?;
            runner.run(&mut self.ctx, instruction);
            println!("after: {}", self.ctx.registers[RegisterType::T0]);
        }
        return Ok(());
    }

    fn get_runner(instruction_type: &InstructionType) -> Result<Box<InstructionRunner>, String> {
        return match instruction_type {
            InstructionType::ADDI => Ok(Box::new(ADDIRunner {})),
            InstructionType::ANDI => Ok(Box::new(ANDIRunner {})),
            InstructionType::SLTI => Ok(Box::new(SLTIRunner {})),
        };
    }
}

struct Context {
    registers: EnumMap<RegisterType, i32>,
}

impl Context {
    fn new() -> Self {
        Context {
            registers: EnumMap::<RegisterType, i32>::new(),
        }
    }
}

trait InstructionRunner {
    fn run(self: Box<Self>, ctx: &mut Context, instruction: &Instruction) -> Result<(), String>;
}

struct ADDIRunner {}

impl InstructionRunner for ADDIRunner {
    fn run(self: Box<Self>, ctx: &mut Context, instruction: &Instruction) -> Result<(), String> {
        let imm = ensure_i32(&instruction.s3)?;
        let s1 = ensure_register(&instruction.s1)?;
        let s2 = ensure_register(&instruction.s2)?;

        ctx.registers[*s1] = ctx.registers[*s2] + imm;
        return Ok(());
    }
}

struct SLTIRunner {}

impl InstructionRunner for SLTIRunner {
    fn run(self: Box<Self>, ctx: &mut Context, instruction: &Instruction) -> Result<(), String> {
        return Ok(());
    }
}

struct ANDIRunner {}

impl InstructionRunner for ANDIRunner {
    fn run(self: Box<Self>, ctx: &mut Context, instruction: &Instruction) -> Result<(), String> {
        return Ok(());
    }
}

fn ensure_register(e: &Either<RegisterType, String>) -> Result<&RegisterType, String> {
    return match e {
        Left(r) => return Ok(r),
        Right(_) => Err("not register type".to_string()),
    };
}

fn ensure_i32(e: &Either<RegisterType, String>) -> Result<i32, String> {
    return match e {
        Right(s) => match s.parse::<i32>() {
            Ok(n) => Ok(n),
            Err(e) => Err(e.to_string()),
        },
        Left(_) => Err("not immediate type".to_string()),
    };
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub s1: Either<RegisterType, String>,
    pub s2: Either<RegisterType, String>,
    pub s3: Either<RegisterType, String>,
}

#[derive(PartialEq, Debug, Enum, Clone, Copy)]
pub enum InstructionType {
    ADDI,
    SLTI,
    ANDI,
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
    use crate::opcodes::RegisterType::ZERO;

    #[test]
    fn test_runner() {
        let instructions = vec![
            Instruction {
                instruction_type: InstructionType::ADDI,
                s1: Left(RegisterType::T0),
                s2: Left(RegisterType::ZERO),
                s3: Right("1".to_string()),
            },
            Instruction {
                instruction_type: InstructionType::ADDI,
                s1: Left(RegisterType::T1),
                s2: Left(RegisterType::T0),
                s3: Right("1".to_string()),
            },
        ];
        let mut runner = Runner::new(instructions);
        runner.run().unwrap();
        assert_eq!(runner.ctx.registers[RegisterType::T1], 2);
    }

    #[test]
    fn test_addi() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;
        let runner = Box::new(ADDIRunner {});
        runner
            .run(
                &mut ctx,
                &Instruction {
                    instruction_type: InstructionType::ADDI,
                    s1: Left(RegisterType::T0),
                    s2: Left(RegisterType::T1),
                    s3: Right("1".to_string()),
                },
            )
            .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 2);
    }
}
