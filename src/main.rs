// use clap::{App as ClapApp, Arg};

fn main() {
    // let args = ClapApp::new("vega")
    //     .arg(
    //         Arg::with_name("file")
    //             .short("f")
    //             .long("file")
    //             .takes_value(true)
    //             .help("RISC file")
    //             .required(true),
    //     )
    //     .get_matches();
    //
    // let file = args.value_of("file").unwrap();
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    s1: String,
    s2: String,
    s3: String,
}

#[derive(Debug)]
enum InstructionType {
    ADDI,
    SLTI,
    ANDI,
}

fn parse(s: String) -> Result<Vec<Instruction>, String> {
    let mut instructions: Vec<Instruction> = vec![];

    for line in s.split("\n") {
        let option = line.find(' ');
        if option.is_none() {
            return Err(format_args!("invalid line: {}", line).to_string());
        }

        let instruction_type_string = &line[..option.unwrap()];
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

        instructions.push(Instruction {
            instruction_type,
            s1: "".to_string(),
            s2: "".to_string(),
            s3: "".to_string(),
        })
    }

    return Ok(instructions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse(
            "addi t0, zero, 10
slti t1, t0, 11
andi t2, t1, 11
addi ra, t2, 2"
                .to_string(),
        );
        println!("{:?}", result.unwrap())
    }
}
