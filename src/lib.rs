#![feature(generator_trait, generators)]

extern crate num;
extern crate num_traits;
#[macro_use]
extern crate num_derive;

pub mod disassembler;
pub mod ops;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
