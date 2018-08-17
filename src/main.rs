#![feature(cell_update)]

extern crate num;
extern crate num_traits;
#[macro_use]
extern crate num_derive;

pub mod machine;
pub fn main() {
    machine::Machine::with_rom("roms/invaders.rom")
        .unwrap()
        .run()
        .unwrap()
}
