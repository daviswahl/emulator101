extern crate num;
extern crate num_traits;
#[macro_use]
extern crate num_derive;

pub mod cpu;
pub mod ops;

pub fn main() {
    cpu::run()
}


