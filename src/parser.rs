use crate::opcodes::{Instruction, InstructionType, RegisterType};
use either::*;

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
            "add" => InstructionType::ADD,
            "addi" => InstructionType::ADDI,
            "and" => InstructionType::AND,
            "andi" => InstructionType::ANDI,
            "auipc" => InstructionType::AUIPC,
            "lui" => InstructionType::LUI,
            "or" => InstructionType::OR,
            "ori" => InstructionType::ORI,
            "slli" => InstructionType::SLLI,
            "slt" => InstructionType::SLT,
            "slti" => InstructionType::SLTI,
            "sltu" => InstructionType::SLTU,
            "srai" => InstructionType::SRAI,
            "sub" => InstructionType::SUB,
            "xor" => InstructionType::XOR,
            "xori" => InstructionType::XORI,
            _ => {
                return Err(
                    format_args!("invalid instruction type: {}", instruction_type_string)
                        .to_string(),
                )
            }
        };

        let remaining_line = &trimmed_line[first_whitespace.unwrap() + 1..];
        let elements: Vec<&str> = remaining_line.split(',').collect();
        if elements.len() <= 1 {
            return Err(format_args!("missing elements: {}", remaining_line).to_string());
        }

        if elements.len() == 2 {
            instructions.push(Instruction {
                instruction_type,
                i1: parse_register(elements[0].trim().to_string()),
                i2: parse_register(elements[1].trim().to_string()),
                i3: Right("".to_string()),
            })
        } else {
            instructions.push(Instruction {
                instruction_type,
                i1: parse_register(elements[0].trim().to_string()),
                i2: parse_register(elements[1].trim().to_string()),
                i3: parse_register(elements[2].trim().to_string()),
            })
        }
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
    use crate::opcodes::{Instruction, InstructionType, RegisterType};

    #[test]
    fn test_parse() {
        let result = parse(
            "addi t0, zero, 10
slti t1, t0, 11
lui t2, 3"
                .to_string(),
        );
        let instructions = result.unwrap();
        assert_eq!(3, instructions.len());
        assert_eq!(
            instructions[0],
            Instruction {
                instruction_type: InstructionType::ADDI,
                i1: Left(RegisterType::T0),
                i2: Left(RegisterType::ZERO),
                i3: Right("10".to_string()),
            }
        );
        assert_eq!(
            instructions[1],
            Instruction {
                instruction_type: InstructionType::SLTI,
                i1: Left(RegisterType::T1),
                i2: Left(RegisterType::T0),
                i3: Right("11".to_string()),
            }
        );
        assert_eq!(
            instructions[2],
            Instruction {
                instruction_type: InstructionType::LUI,
                i1: Left(RegisterType::T2),
                i2: Right("3".to_string()),
                i3: Right("".to_string()),
            }
        );
    }
}
