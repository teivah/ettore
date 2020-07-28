use either::*;
use enum_map::EnumMap;

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
    fn add(&self, ctx: &mut Context, instruction: Instruction) -> Result<(), String>;
}

struct ADDIRunner {}

impl InstructionRunner for ADDIRunner {
    fn add(&self, ctx: &mut Context, instruction: Instruction) -> Result<(), String> {
        let imm = ensure_i32(instruction.s3)?;
        let s1 = ensure_register(instruction.s1)?;
        let s2 = ensure_register(instruction.s2)?;

        ctx.registers[s1] = ctx.registers[s2] + imm;
        return Ok(());
    }
}

fn ensure_register(e: Either<RegisterType, String>) -> Result<RegisterType, String> {
    return match e {
        Left(r) => return Ok(r),
        Right(_) => Err("not register type".to_string()),
    };
}

fn ensure_i32(e: Either<RegisterType, String>) -> Result<i32, String> {
    return match e {
        Right(s) => match s.parse::<i32>() {
            Ok(n) => Ok(n),
            Err(e) => Err(e.to_string()),
        },
        Left(_) => Err("not immediate type".to_string()),
    };
}

#[derive(PartialEq, Debug)]
struct Instruction {
    instruction_type: InstructionType,
    s1: Either<RegisterType, String>,
    s2: Either<RegisterType, String>,
    s3: Either<RegisterType, String>,
}

#[derive(PartialEq, Debug)]
enum InstructionType {
    ADDI,
    SLTI,
    ANDI,
}

#[derive(PartialEq, Debug, EnumMap)]
enum RegisterType {
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

fn parse(s: String) -> Result<Vec<Instruction>, String> {
    let mut instructions: Vec<Instruction> = vec![];

    for line in s.split("\n") {
        let trimmed_line = line.trim();
        let first_whitespace = trimmed_line.find(' ');
        if first_whitespace.is_none() {
            return Err(format_args!("invalid line: {}", trimmed_line).to_string());
        }

        let instruction_type_string = &trimmed_line[..first_whitespace.unwrap()];
        let instruction_type = match instruction_type_string {
            "addi" => InstructionType::ADDI,
            "slti" => InstructionType::SLTI,
            "andi" => InstructionType::ANDI,
            _ => {
                return Err(
                    format_args!("invalid instruction type: {}", instruction_type_string)
                        .to_string(),
                )
            }
        };

        let remaining_line = &trimmed_line[first_whitespace.unwrap() + 1..];
        let elements: Vec<&str> = remaining_line.split(',').collect();
        if elements.len() != 3 {
            return Err(format_args!("missing elements: {}", remaining_line).to_string());
        }

        instructions.push(Instruction {
            instruction_type,
            s1: parse_register(elements[0].trim().to_string()),
            s2: parse_register(elements[1].trim().to_string()),
            s3: parse_register(elements[2].trim().to_string()),
        })
    }

    return Ok(instructions);
}

fn parse_register(s: String) -> Either<RegisterType, String> {
    return match s.as_str() {
        "zero" => Left(RegisterType::ZERO),
        "ra" => Left(RegisterType::RA),
        "sp" => Left(RegisterType::SP),
        "gp" => Left(RegisterType::GP),
        "tp" => Left(RegisterType::TP),
        "t0" => Left(RegisterType::T0),
        "t1" => Left(RegisterType::T1),
        "t2" => Left(RegisterType::T2),
        "s0" => Left(RegisterType::S0),
        "s1" => Left(RegisterType::S1),
        "a0" => Left(RegisterType::A0),
        "a1" => Left(RegisterType::A1),
        "a2" => Left(RegisterType::A2),
        "a3" => Left(RegisterType::A3),
        "a4" => Left(RegisterType::A4),
        "a5" => Left(RegisterType::A5),
        "a6" => Left(RegisterType::A6),
        "a7" => Left(RegisterType::A7),
        "s2" => Left(RegisterType::S2),
        "s3" => Left(RegisterType::S3),
        "s4" => Left(RegisterType::S4),
        "s5" => Left(RegisterType::S5),
        "s6" => Left(RegisterType::S6),
        "s7" => Left(RegisterType::S7),
        "s8" => Left(RegisterType::S8),
        "s9" => Left(RegisterType::S9),
        "s10" => Left(RegisterType::S10),
        "s11" => Left(RegisterType::S11),
        "t3" => Left(RegisterType::T3),
        "t4" => Left(RegisterType::T4),
        "t5" => Left(RegisterType::T5),
        "t6" => Left(RegisterType::T6),
        _ => {
            return Right(s);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcodes::RegisterType::ZERO;

    #[test]
    fn test_parse() {
        let result = parse(
            "addi t0, zero, 10
slti t1, t0, 11"
                .to_string(),
        );
        let instructions = result.unwrap();
        assert_eq!(2, instructions.len());
        assert_eq!(
            instructions[0],
            Instruction {
                instruction_type: InstructionType::ADDI,
                s1: Left(RegisterType::T0),
                s2: Left(RegisterType::ZERO),
                s3: Right("10".to_string())
            }
        );
        assert_eq!(
            instructions[1],
            Instruction {
                instruction_type: InstructionType::SLTI,
                s1: Left(RegisterType::T1),
                s2: Left(RegisterType::T0),
                s3: Right("11".to_string())
            }
        );
    }

    #[test]
    fn test_context_new() {
        let context = Context::new();
        assert_eq!(0, context.registers[ZERO]);
    }

    #[test]
    fn test_addi() {
        let mut ctx = Context::new();
        ctx.registers[RegisterType::T1] = 1;
        let runner = ADDIRunner {};
        runner
            .add(
                &mut ctx,
                Instruction {
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
