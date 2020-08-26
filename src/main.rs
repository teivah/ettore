mod mvm3;

use crate::opcodes::Application;
#[macro_use]
extern crate queues;

use log::info;

mod bit;
mod mvm1;
mod mvm2;
mod opcodes;
mod parser;

pub const I5_7360U: i64 = 2_300_000_000;
pub const SECOND_TO_NANOSECOND: i64 = 1_000_000_000;

trait VirtualMachine {
    fn run(&mut self, application: &Application) -> Result<f32, String>;
}

fn main() {}

#[cfg(test)]
mod testdads {
    use super::*;
    use crate::mvm1::Mvm1;
    use crate::mvm2::Mvm2;
    use crate::mvm3::Mvm3;
    use crate::parser::parse;
    use std::borrow::Borrow;
    use std::fs;

    fn execute(vm: &mut dyn VirtualMachine, instructions: &str) -> Result<f32, String> {
        let application = parse(instructions.to_string()).unwrap();
        return vm.run(&application);
    }

    fn stats(test: &str, cycles: f32) {
        let s = cycles / I5_7360U as f32;
        let ns = s * SECOND_TO_NANOSECOND as f32;
        info!("{}: {} cycles, {:.2} nanoseconds", test, cycles, ns);
    }

    #[test]
    fn test_mvm1_prime_number() {
        let mut vm = Mvm1::new(5);
        let cycles = execute(
            &mut vm,
            fs::read_to_string("res/risc/prime-number-1109.asm")
                .unwrap()
                .as_str()
                .borrow(),
        )
        .unwrap();
        stats("mvm1 - prime number", cycles);
    }

    #[test]
    fn test_mvm2_prime_number() {
        let mut vm = Mvm2::new(5);
        let cycles = execute(
            &mut vm,
            fs::read_to_string("res/risc/prime-number-1109.asm")
                .unwrap()
                .as_str()
                .borrow(),
        )
        .unwrap();
        stats("mvm2 - prime number", cycles);
    }

    #[test]
    fn test_mvm3_prime_number() {
        let mut vm = Mvm3::new(5);

        let application = parse(
            fs::read_to_string("res/risc/prime-number-1109.asm")
                .unwrap()
                .as_str()
                .borrow()
                .to_string(),
        )
        .unwrap();
        let cycles = vm.run(&application).unwrap();
        stats("mvm3 - prime number", cycles);
    }
}
