mod bit;
mod mvm1;
mod opcodes;
mod parser;

// use clap::{App as ClapApp, Arg};

pub const I5_7360U: i64 = 2_300_000_000;
pub const SECOND_TO_NANOSECOND: i64 = 1_000_000_000;

fn main() {
    // let args = ClapApp::new("majorana")
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
