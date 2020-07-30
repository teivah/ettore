mod bit;
mod mvm1;
mod mvm2;
mod opcodes;
mod parser;

pub const I5_7360U: i64 = 2_300_000_000;
pub const SECOND_TO_NANOSECOND: i64 = 1_000_000_000;

trait VirtualMachine {
    fn run(&mut self) -> Result<(), String>;
}

fn main() {}
