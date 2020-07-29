use crate::opcodes::{Add, InstructionRunner, RegisterType};

fn parse(s: String) -> Result<Vec<Box<dyn InstructionRunner>>, String> {
    let mut instructions: Vec<Box<dyn InstructionRunner>> = vec![];

    for line in s.split("\n") {
        let trimmed_line = line.trim();
        let first_whitespace = trimmed_line.find(' ');
        if first_whitespace.is_none() {
            return Err(format_args!("invalid line: {}", trimmed_line).to_string());
        }

        // TODO parse labels
        let instruction_type_string = &trimmed_line[..first_whitespace.unwrap()];
        let remaining_line = &trimmed_line[first_whitespace.unwrap() + 1..];
        let elements: Vec<&str> = remaining_line.split(',').collect();
        let instruction: Box<dyn InstructionRunner> = match instruction_type_string {
            "add" => {
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Add { rd, rs1, rs2 })
            }
            // "addi" => InstructionType::ADDI,
            // "and" => InstructionType::AND,
            // "andi" => InstructionType::ANDI,
            // "auipc" => InstructionType::AUIPC,
            // "jal" => InstructionType::JAL,
            // "lui" => InstructionType::LUI,
            // "nop" => InstructionType::NOP,
            // "or" => InstructionType::OR,
            // "ori" => InstructionType::ORI,
            // "sll" => InstructionType::SLL,
            // "slli" => InstructionType::SLLI,
            // "slt" => InstructionType::SLT,
            // "slti" => InstructionType::SLTI,
            // "sltu" => InstructionType::SLTU,
            // "sra" => InstructionType::SRA,
            // "srai" => InstructionType::SRAI,
            // "srl" => InstructionType::SRL,
            // "srli" => InstructionType::SRLI,
            // "sub" => InstructionType::SUB,
            // "xor" => InstructionType::XOR,
            // "xori" => InstructionType::XORI,
            _ => {
                return Err(
                    format_args!("invalid instruction type: {}", instruction_type_string)
                        .to_string(),
                )
            }
        };
        instructions.push(instruction);

        // let remaining_line = &trimmed_line[first_whitespace.unwrap() + 1..];
        // let elements: Vec<&str> = remaining_line.split(',').collect();
        // if elements.len() <= 1 {
        //     return Err(format_args!("missing elements: {}", remaining_line).to_string());
        // }
        //
        // if elements.len() == 0 {
        //     instructions.push(Instruction {
        //         instruction_type,
        //         i1: Right("".to_string()),
        //         i2: Right("".to_string()),
        //         i3: Right("".to_string()),
        //     })
        // } else if elements.len() == 1 {
        //     instructions.push(Instruction {
        //         instruction_type,
        //         i1: parse_input(elements[0].trim().to_string()),
        //         i2: Right("".to_string()),
        //         i3: Right("".to_string()),
        //     })
        // } else if elements.len() == 2 {
        //     instructions.push(Instruction {
        //         instruction_type,
        //         i1: parse_input(elements[0].trim().to_string()),
        //         i2: parse_input(elements[1].trim().to_string()),
        //         i3: Right("".to_string()),
        //     })
        // } else {
        //     instructions.push(Instruction {
        //         instruction_type,
        //         i1: parse_input(elements[0].trim().to_string()),
        //         i2: parse_input(elements[1].trim().to_string()),
        //         i3: parse_input(elements[2].trim().to_string()),
        //     })
        // }
    }

    return Ok(instructions);
}

fn parse_register(s: String) -> Result<RegisterType, String> {
    return match s.as_str() {
        "zero" => Ok(RegisterType::ZERO),
        "ra" => Ok(RegisterType::RA),
        "sp" => Ok(RegisterType::SP),
        "gp" => Ok(RegisterType::GP),
        "tp" => Ok(RegisterType::TP),
        "t0" => Ok(RegisterType::T0),
        "t1" => Ok(RegisterType::T1),
        "t2" => Ok(RegisterType::T2),
        "s0" => Ok(RegisterType::S0),
        "s1" => Ok(RegisterType::S1),
        "a0" => Ok(RegisterType::A0),
        "a1" => Ok(RegisterType::A1),
        "a2" => Ok(RegisterType::A2),
        "a3" => Ok(RegisterType::A3),
        "a4" => Ok(RegisterType::A4),
        "a5" => Ok(RegisterType::A5),
        "a6" => Ok(RegisterType::A6),
        "a7" => Ok(RegisterType::A7),
        "s2" => Ok(RegisterType::S2),
        "s3" => Ok(RegisterType::S3),
        "s4" => Ok(RegisterType::S4),
        "s5" => Ok(RegisterType::S5),
        "s6" => Ok(RegisterType::S6),
        "s7" => Ok(RegisterType::S7),
        "s8" => Ok(RegisterType::S8),
        "s9" => Ok(RegisterType::S9),
        "s10" => Ok(RegisterType::S10),
        "s11" => Ok(RegisterType::S11),
        "t3" => Ok(RegisterType::T3),
        "t4" => Ok(RegisterType::T4),
        "t5" => Ok(RegisterType::T5),
        "t6" => Ok(RegisterType::T6),
        _ => Err(format_args!("unknown register: {}", s).to_string()),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        //         let result = parse(
        //             "addi t0, zero, 10
        // slti t1, t0, 11
        // lui t2, 3"
        //                 .to_string(),
        //         );
        let result = parse(
            "add t0, t1, t2
add t1, t2, t0"
                .to_string(),
        );
        let instructions = result.unwrap();
        assert_eq!(2, instructions.len());
        // assert_eq!(
        //     instructions[1],
        //     Add {
        //         rd: RegisterType::T1,
        //         rs1: RegisterType::T2,
        //         rs2: RegisterType::T0
        //     }
        // );
    }
}
