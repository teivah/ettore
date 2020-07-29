use crate::opcodes::*;

fn parse(s: String) -> Result<Application, String> {
    let mut instructions: Vec<Box<dyn InstructionRunner>> = vec![];
    let mut labels: Vec<String> = vec![];

    for line in s.split("\n") {
        let trimmed_line = line.trim();
        if trimmed_line.len() == 0 {
            continue;
        }

        let first_whitespace = trimmed_line.find(' ');
        let last_character = trimmed_line.chars().last().unwrap();
        if first_whitespace.is_none() && last_character != ':' {
            return Err(format_args!("invalid line: {}", trimmed_line).to_string());
        } else if first_whitespace.is_none() && last_character == ':' {
            labels.push(trimmed_line[..trimmed_line.len() - 1].to_string());
            continue;
        }

        let instruction_type_string = &trimmed_line[..first_whitespace.unwrap()];
        let remaining_line = &trimmed_line[first_whitespace.unwrap() + 1..];
        let elements: Vec<&str> = remaining_line.split(',').collect();
        let instruction: Box<dyn InstructionRunner> = match instruction_type_string {
            "add" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Add { rd, rs1, rs2 })
            }
            "addi" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let imm = i32(elements[2].trim().to_string())?;
                Box::new(Addi { rd, rs, imm })
            }
            "and" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(And { rd, rs1, rs2 })
            }
            "andi" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let imm = i32(elements[2].trim().to_string())?;
                Box::new(Andi { rd, rs, imm })
            }
            "auipc" => {
                validate_args(2, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let imm = i32(elements[1].trim().to_string())?;
                Box::new(Auipc { rd, imm })
            }
            "jal" => {
                validate_args(2, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let label = elements[1].trim().to_string();
                Box::new(Jal { rd, label })
            }
            "lui" => {
                validate_args(2, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let imm = i32(elements[1].trim().to_string())?;
                Box::new(Lui { rd, imm })
            }
            "nop" => Box::new(Nop {}),
            "or" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Or { rd, rs1, rs2 })
            }
            "ori" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let imm = i32(elements[2].trim().to_string())?;
                Box::new(Ori { rd, rs, imm })
            }
            "sll" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Sll { rd, rs1, rs2 })
            }
            "slli" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let imm = i32(elements[2].trim().to_string())?;
                Box::new(Slli { rd, rs, imm })
            }
            "slt" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Slt { rd, rs1, rs2 })
            }
            "slti" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let imm = i32(elements[2].trim().to_string())?;
                Box::new(Slti { rd, rs, imm })
            }
            "sltu" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let imm = i32(elements[2].trim().to_string())?;
                Box::new(Slti { rd, rs, imm })
            }
            "sra" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Sra { rd, rs1, rs2 })
            }
            "srai" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let imm = i32(elements[2].trim().to_string())?;
                Box::new(Srai { rd, rs, imm })
            }
            "srl" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Srl { rd, rs1, rs2 })
            }
            "srli" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let imm = i32(elements[2].trim().to_string())?;
                Box::new(Srli { rd, rs, imm })
            }
            "sub" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Sub { rd, rs1, rs2 })
            }
            "xor" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Xor { rd, rs1, rs2 })
            }
            "xori" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let imm = i32(elements[2].trim().to_string())?;
                Box::new(Xori { rd, rs, imm })
            }
            _ => {
                return Err(
                    format_args!("invalid instruction type: {}", instruction_type_string)
                        .to_string(),
                )
            }
        };
        instructions.push(instruction);
    }

    return Ok(Application {
        instructions,
        labels,
    });
}

fn validate_args(expected: usize, args: &Vec<&str>, line: &str) -> Result<(), String> {
    if args.len() == expected {
        return Ok(());
    }

    return Err(format_args!(
        "invalid line expected {} arguments, got {}: {}",
        expected,
        args.len(),
        line
    )
    .to_string());
}

fn i32(s: String) -> Result<i32, String> {
    return match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(e.to_string()),
    };
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
        let result = parse(
            "    jal t0, foo
    addi t1, zero, 1
foo:
    addi t2, zero, 2"
                .to_string(),
        );
        let application = result.unwrap();
        assert_eq!(3, application.instructions.len());
        assert_eq!(1, application.labels.len());
        assert_eq!("foo", application.labels[0]);
    }
}
