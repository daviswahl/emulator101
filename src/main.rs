extern crate num;
extern crate num_traits;
#[macro_use]
extern crate num_derive;

pub mod disassembler;
pub mod emulator;
pub mod ops;

pub fn main() {
    emulator::run()
}

#[cfg(test)]
mod tests {
    use emulator;
    use std::char;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_diag() {
        assert_eq!(emulator::diag(), Err("exit".to_string()))
    }
}
