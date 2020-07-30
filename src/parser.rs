use crate::opcodes::*;
use std::collections::HashMap;

pub fn parse(s: String) -> Result<Application, String> {
    let mut instructions: Vec<Box<dyn InstructionRunner>> = vec![];
    let mut labels = HashMap::new();
    let mut pc: i32 = 0;

    for line in s.split("\n") {
        let trimmed_line = line.trim();
        if trimmed_line.len() == 0 || trimmed_line.chars().next().unwrap() == '#' {
            continue;
        }

        let first_whitespace = trimmed_line.find(' ');
        let last_character = trimmed_line.chars().last().unwrap();
        if first_whitespace.is_none() && last_character != ':' {
            return Err(format_args!("invalid line: {}", trimmed_line).to_string());
        } else if first_whitespace.is_none() && last_character == ':' {
            labels.insert(trimmed_line[..trimmed_line.len() - 1].to_string(), pc);
            continue;
        }

        let mut remaining_line = &trimmed_line[first_whitespace.unwrap() + 1..];
        let comment = remaining_line.find('#');
        match comment {
            None => (),
            Some(i) => remaining_line = &remaining_line[..i].trim(),
        }

        let elements: Vec<&str> = remaining_line.split(',').collect();

        let instruction: Box<dyn InstructionRunner> = match trimmed_line
            [..first_whitespace.unwrap()]
            .to_lowercase()
            .as_str()
        {
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
            "beq" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let label = elements[2].trim().to_string();
                Box::new(Beq {
                    rs1: rd,
                    rs2: rs,
                    label,
                })
            }
            "bge" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let label = elements[2].trim().to_string();
                Box::new(Bge {
                    rs1: rd,
                    rs2: rs,
                    label,
                })
            }
            "bgeu" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let label = elements[2].trim().to_string();
                Box::new(Bgeu {
                    rs1: rd,
                    rs2: rs,
                    label,
                })
            }
            "blt" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let label = elements[2].trim().to_string();
                Box::new(Blt {
                    rs1: rd,
                    rs2: rs,
                    label,
                })
            }
            "bltu" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let label = elements[2].trim().to_string();
                Box::new(Bltu {
                    rs1: rd,
                    rs2: rs,
                    label,
                })
            }
            "bne" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let label = elements[2].trim().to_string();
                Box::new(Bne {
                    rs1: rd,
                    rs2: rs,
                    label,
                })
            }
            "div" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Div { rd, rs1, rs2 })
            }
            "jal" => {
                validate_args(2, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let label = elements[1].trim().to_string();
                Box::new(Jal { rd, label })
            }
            "jalr" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs = parse_register(elements[1].trim().to_string())?;
                let imm = i32(elements[2].trim().to_string())?;
                Box::new(Jalr { rd, rs, imm })
            }
            "lui" => {
                validate_args(2, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let imm = i32(elements[1].trim().to_string())?;
                Box::new(Lui { rd, imm })
            }
            "lb" => {
                validate_args(3, &elements, remaining_line)?;
                let rs2 = parse_register(elements[0].trim().to_string())?;
                let offset = i32(elements[1].trim().to_string())?;
                let rs1 = parse_register(elements[2].trim().to_string())?;
                Box::new(Lb { rs2, offset, rs1 })
            }
            "lh" => {
                validate_args(3, &elements, remaining_line)?;
                let rs2 = parse_register(elements[0].trim().to_string())?;
                let offset = i32(elements[1].trim().to_string())?;
                let rs1 = parse_register(elements[2].trim().to_string())?;
                Box::new(Lh { rs2, offset, rs1 })
            }
            "lw" => {
                validate_args(3, &elements, remaining_line)?;
                let rs2 = parse_register(elements[0].trim().to_string())?;
                let offset = i32(elements[1].trim().to_string())?;
                let rs1 = parse_register(elements[2].trim().to_string())?;
                Box::new(Lw { rs2, offset, rs1 })
            }
            "nop" => Box::new(Nop {}),
            "mul" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Mul { rd, rs1, rs2 })
            }
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
            "rem" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Rem { rd, rs1, rs2 })
            }
            "sb" => {
                validate_args_interval(2, 3, &elements, remaining_line)?;
                if elements.len() == 2 {
                    let rs2 = parse_register(elements[0].trim().to_string())?;
                    let imm_reg = parse_offset_reg(elements[1].to_string())?;
                    Box::new(Sb {
                        rs2,
                        offset: imm_reg.0,
                        rs1: imm_reg.1,
                    })
                } else {
                    let rs2 = parse_register(elements[0].trim().to_string())?;
                    let offset = i32(elements[1].trim().to_string())?;
                    let rs1 = parse_register(elements[2].trim().to_string())?;
                    Box::new(Sb { rs2, offset, rs1 })
                }
            }
            "sh" => {
                validate_args(3, &elements, remaining_line)?;
                let rs2 = parse_register(elements[0].trim().to_string())?;
                let offset = i32(elements[1].trim().to_string())?;
                let rs1 = parse_register(elements[2].trim().to_string())?;
                Box::new(Sh { rs2, offset, rs1 })
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
            "sltu" => {
                validate_args(3, &elements, remaining_line)?;
                let rd = parse_register(elements[0].trim().to_string())?;
                let rs1 = parse_register(elements[1].trim().to_string())?;
                let rs2 = parse_register(elements[2].trim().to_string())?;
                Box::new(Sltu { rd, rs1, rs2 })
            }
            "slti" => {
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
            "sw" => {
                validate_args(3, &elements, remaining_line)?;
                let rs2 = parse_register(elements[0].trim().to_string())?;
                let offset = i32(elements[1].trim().to_string())?;
                let rs1 = parse_register(elements[2].trim().to_string())?;
                Box::new(Sw { rs2, offset, rs1 })
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
            _ => return Err(format_args!("invalid instruction type: {}", trimmed_line).to_string()),
        };
        instructions.push(instruction);
        pc += 4;
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

fn validate_args_interval(
    min: usize,
    max: usize,
    args: &Vec<&str>,
    line: &str,
) -> Result<(), String> {
    if args.len() >= min && args.len() <= max {
        return Ok(());
    }

    return Err(format_args!(
        "invalid line expected between {} and {} arguments, got {}: {}",
        min,
        max,
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

fn parse_offset_reg(s: String) -> Result<(i32, RegisterType), String> {
    let first_parenthesis = s.find('(');
    let first_parenthesis_idx: usize;
    match first_parenthesis {
        None => return Err(format_args!("invalid offset register: {}", s).to_string()),
        Some(i) => first_parenthesis_idx = i,
    };

    let imm_string = s[..first_parenthesis_idx].trim();
    let imm = i32(imm_string.to_string())?;

    let reg_string = s[first_parenthesis_idx + 1..s.len() - 1].trim();

    return Ok((imm, parse_register(reg_string.to_string())?));
}

fn parse_register(s: String) -> Result<RegisterType, String> {
    return match s.as_str() {
        "zero" | "$zero" => Ok(RegisterType::ZERO),
        "ra" | "$ra" => Ok(RegisterType::RA),
        "sp" | "$sp" => Ok(RegisterType::SP),
        "gp" | "$gp" => Ok(RegisterType::GP),
        "tp" | "$tp" => Ok(RegisterType::TP),
        "t0" | "$t0" => Ok(RegisterType::T0),
        "t1" | "$t1" => Ok(RegisterType::T1),
        "t2" | "$t2" => Ok(RegisterType::T2),
        "s0" | "$s0" => Ok(RegisterType::S0),
        "s1" | "$s1" => Ok(RegisterType::S1),
        "a0" | "$a0" => Ok(RegisterType::A0),
        "a1" | "$a1" => Ok(RegisterType::A1),
        "a2" | "$a2" => Ok(RegisterType::A2),
        "a3" | "$a3" => Ok(RegisterType::A3),
        "a4" | "$a4" => Ok(RegisterType::A4),
        "a5" | "$a5" => Ok(RegisterType::A5),
        "a6" | "$a6" => Ok(RegisterType::A6),
        "a7" | "$a7" => Ok(RegisterType::A7),
        "s2" | "$s2" => Ok(RegisterType::S2),
        "s3" | "$s3" => Ok(RegisterType::S3),
        "s4" | "$s4" => Ok(RegisterType::S4),
        "s5" | "$s5" => Ok(RegisterType::S5),
        "s6" | "$s6" => Ok(RegisterType::S6),
        "s7" | "$s7" => Ok(RegisterType::S7),
        "s8" | "$s8" => Ok(RegisterType::S8),
        "s9" | "$s9" => Ok(RegisterType::S9),
        "s10" | "$s10" => Ok(RegisterType::S10),
        "s11" | "$s11" => Ok(RegisterType::S11),
        "t3" | "$t3" => Ok(RegisterType::T3),
        "t4" | "$t4" => Ok(RegisterType::T4),
        "t5" | "$t5" => Ok(RegisterType::T5),
        "t6" | "$t6" => Ok(RegisterType::T6),
        _ => Err(format_args!("unknown register: {}", s).to_string()),
    };
}
