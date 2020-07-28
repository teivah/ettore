use either::*;
use enum_map::{Enum, EnumMap};

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
        let mut address: i32 = 0;
        for instruction in &self.instructions {
            let runner = Runner::get_runner(&instruction.instruction_type);
            runner(&mut self.ctx, instruction, address)?;
            address += 4;
        }
        return Ok(());
    }

    fn get_runner(
        instruction_type: &InstructionType,
    ) -> fn(ctx: &mut Context, instruction: &Instruction, addr: i32) -> Result<(), String> {
        return match instruction_type {
            InstructionType::ADD => add,
            InstructionType::ADDI => addi,
            InstructionType::ANDI => andi,
            InstructionType::AUIPC => auipc,
            InstructionType::LUI => lui,
            InstructionType::ORI => ori,
            InstructionType::SLLI => slli,
            InstructionType::SLT => slt,
            InstructionType::SLTI => slti,
            InstructionType::SLTU => slt,
            InstructionType::SRAI => srli,
            InstructionType::SRLI => srli,
            InstructionType::SUB => sub,
            InstructionType::XORI => xori,
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

fn add(ctx: &mut Context, instruction: &Instruction, _: i32) -> Result<(), String> {
    let rd = register(&instruction.i1)?;
    let rs1 = register(&instruction.i2)?;
    let rs2 = register(&instruction.i3)?;

    ctx.registers[*rd] = ctx.registers[*rs1] + ctx.registers[*rs2];
    return Ok(());
}

fn addi(ctx: &mut Context, instruction: &Instruction, _: i32) -> Result<(), String> {
    let imm = i32(&instruction.i3)?;
    let rd = register(&instruction.i1)?;
    let rs = register(&instruction.i2)?;

    ctx.registers[*rd] = ctx.registers[*rs] + imm;
    return Ok(());
}

fn andi(ctx: &mut Context, instruction: &Instruction, _: i32) -> Result<(), String> {
    let imm = i32(&instruction.i3)?;
    let rd = register(&instruction.i1)?;
    let rs = register(&instruction.i2)?;

    ctx.registers[*rd] = ctx.registers[*rs] & imm;
    return Ok(());
}

fn auipc(ctx: &mut Context, instruction: &Instruction, address: i32) -> Result<(), String> {
    let imm = i32(&instruction.i2)?;
    let rd = register(&instruction.i1)?;

    ctx.registers[*rd] = address + (imm << 12);
    return Ok(());
}

fn lui(ctx: &mut Context, instruction: &Instruction, _: i32) -> Result<(), String> {
    let imm = i32(&instruction.i2)?;
    let rd = register(&instruction.i1)?;

    ctx.registers[*rd] = imm << 12;
    return Ok(());
}

fn ori(ctx: &mut Context, instruction: &Instruction, _: i32) -> Result<(), String> {
    let imm = i32(&instruction.i3)?;
    let rd = register(&instruction.i1)?;
    let rs = register(&instruction.i2)?;

    ctx.registers[*rd] = ctx.registers[*rs] | imm;
    return Ok(());
}

fn slli(ctx: &mut Context, instruction: &Instruction, _: i32) -> Result<(), String> {
    let imm = i32(&instruction.i3)?;
    let rd = register(&instruction.i1)?;
    let rs = register(&instruction.i2)?;

    ctx.registers[*rd] = ctx.registers[*rs] << imm;
    return Ok(());
}

fn slt(ctx: &mut Context, instruction: &Instruction, _: i32) -> Result<(), String> {
    let rd = register(&instruction.i1)?;
    let rs1 = register(&instruction.i2)?;
    let rs2 = register(&instruction.i3)?;

    if ctx.registers[*rs1] < ctx.registers[*rs2] {
        ctx.registers[*rd] = 1
    } else {
        ctx.registers[*rd] = 0
    }
    return Ok(());
}

fn slti(ctx: &mut Context, instruction: &Instruction, _: i32) -> Result<(), String> {
    let imm = i32(&instruction.i3)?;
    let rd = register(&instruction.i1)?;
    let rs = register(&instruction.i2)?;

    if ctx.registers[*rs] < imm {
        ctx.registers[*rd] = 1
    } else {
        ctx.registers[*rd] = 0
    }
    return Ok(());
}

fn srli(ctx: &mut Context, instruction: &Instruction, _: i32) -> Result<(), String> {
    let imm = i32(&instruction.i3)?;
    let rd = register(&instruction.i1)?;
    let rs = register(&instruction.i2)?;

    ctx.registers[*rd] = ctx.registers[*rs] >> imm;
    return Ok(());
}

fn sub(ctx: &mut Context, instruction: &Instruction, _: i32) -> Result<(), String> {
    let rd = register(&instruction.i1)?;
    let rs1 = register(&instruction.i2)?;
    let rs2 = register(&instruction.i3)?;

    ctx.registers[*rd] = ctx.registers[*rs1] - ctx.registers[*rs2];
    return Ok(());
}

fn xori(ctx: &mut Context, instruction: &Instruction, _: i32) -> Result<(), String> {
    let imm = i32(&instruction.i3)?;
    let rd = register(&instruction.i1)?;
    let rs = register(&instruction.i2)?;

    ctx.registers[*rd] = ctx.registers[*rs] ^ imm;
    return Ok(());
}

fn register(e: &Either<RegisterType, String>) -> Result<&RegisterType, String> {
    return match e {
        Left(r) => return Ok(r),
        Right(_) => Err("not register type".to_string()),
    };
}

fn i32(e: &Either<RegisterType, String>) -> Result<i32, String> {
    return match e {
        Right(s) => match s.parse::<i32>() {
            Ok(n) => Ok(n),
            Err(e) => Err(e.to_string()),
        },
        Left(_) => Err("not integer type".to_string()),
    };
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
    ANDI,
    AUIPC,
    LUI,
    ORI,
    SLLI,
    SLT,
    SLTI,
    SLTU,
    SRAI,
    SRLI,
    SUB,
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
        let instructions = vec![
            Instruction {
                instruction_type: InstructionType::ADDI,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::ZERO),
                i3: Right("1".to_string()),
            },
            Instruction {
                instruction_type: InstructionType::ADDI,
                i1: Left(RegisterType::T1),
                i2: Left(RegisterType::T0),
                i3: Right("1".to_string()),
            },
        ];
        let mut runner = Runner::new(instructions);
        runner.run().unwrap();
        assert_eq!(runner.ctx.registers[RegisterType::T1], 2);
    }

    #[test]
    fn test_add() {
        let instruction = InstructionType::ADD;
        let runner = Runner::get_runner(&instruction);
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;
        ctx.registers[RegisterType::T2] = 2;

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::T1),
                i3: Left(RegisterType::T2),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 3);
    }

    #[test]
    fn test_addi() {
        let instruction = InstructionType::ADDI;
        let runner = Runner::get_runner(&instruction);
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::T1),
                i3: Right("1".to_string()),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 2);
    }

    #[test]
    fn test_auipc() {
        let mut instructions = vec![
            Instruction {
                instruction_type: InstructionType::AUIPC,
                i1: Left(RegisterType::T0),
                i2: Right("0".to_string()),
                i3: Right("".to_string()),
            },
            Instruction {
                instruction_type: InstructionType::AUIPC,
                i1: Left(RegisterType::T0),
                i2: Right("0".to_string()),
                i3: Right("".to_string()),
            },
            Instruction {
                instruction_type: InstructionType::AUIPC,
                i1: Left(RegisterType::T0),
                i2: Right("0".to_string()),
                i3: Right("".to_string()),
            },
        ];
        let mut runner = Runner::new(instructions);
        runner.run().unwrap();
        assert_eq!(runner.ctx.registers[RegisterType::T0], 8);

        instructions = vec![
            Instruction {
                instruction_type: InstructionType::AUIPC,
                i1: Left(RegisterType::T0),
                i2: Right("1".to_string()),
                i3: Right("".to_string()),
            },
            Instruction {
                instruction_type: InstructionType::AUIPC,
                i1: Left(RegisterType::T0),
                i2: Right("1".to_string()),
                i3: Right("".to_string()),
            },
            Instruction {
                instruction_type: InstructionType::AUIPC,
                i1: Left(RegisterType::T0),
                i2: Right("1".to_string()),
                i3: Right("".to_string()),
            },
        ];
        runner = Runner::new(instructions);
        runner.run().unwrap();
        assert_eq!(runner.ctx.registers[RegisterType::T0], 4104);
    }

    #[test]
    fn test_lui() {
        let instruction = InstructionType::LUI;
        let runner = Runner::get_runner(&instruction);
        let mut ctx = Context::new();

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Right("0".to_string()),
                i3: Right("".to_string()),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 0);

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Right("1".to_string()),
                i3: Right("".to_string()),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 4096);

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Right("3".to_string()),
                i3: Right("".to_string()),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 12288);
    }

    #[test]
    fn test_ori() {
        let instruction = InstructionType::ORI;
        let runner = Runner::get_runner(&instruction);
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::T1),
                i3: Right("2".to_string()),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 3);
    }

    #[test]
    fn test_slli() {
        let instruction = InstructionType::SLLI;
        let runner = Runner::get_runner(&instruction);
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::T1),
                i3: Right("2".to_string()),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 4);
    }

    #[test]
    fn test_slt() {
        let instruction = InstructionType::SLT;
        let runner = Runner::get_runner(&instruction);
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 2;
        ctx.registers[RegisterType::T2] = 3;

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::T1),
                i3: Left(RegisterType::T2),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_slti() {
        let instruction = InstructionType::SLTI;
        let runner = Runner::get_runner(&instruction);
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::T1),
                i3: Right("5".to_string()),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_srai() {
        let instruction = InstructionType::SRAI;
        let runner = Runner::get_runner(&instruction);
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 2;

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::T1),
                i3: Right("1".to_string()),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_srli() {
        let instruction = InstructionType::SRLI;
        let runner = Runner::get_runner(&instruction);
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 4;

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::T1),
                i3: Right("2".to_string()),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 1);
    }

    #[test]
    fn test_sub() {
        let instruction = InstructionType::SUB;
        let runner = Runner::get_runner(&instruction);
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 10;
        ctx.registers[RegisterType::T2] = 6;

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::T1),
                i3: Left(RegisterType::T2),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 4);
    }

    #[test]
    fn test_xori() {
        let instruction = InstructionType::XORI;
        let runner = Runner::get_runner(&instruction);
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 3;

        runner(
            &mut ctx,
            &Instruction {
                instruction_type: instruction,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::T1),
                i3: Right("4".to_string()),
            },
            0,
        )
        .unwrap();
        assert_eq!(ctx.registers[RegisterType::T0], 7);
    }
}
