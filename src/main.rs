#![feature(cell_update)]

extern crate num;
extern crate num_traits;
#[macro_use]
extern crate num_derive;

mod space_invaders;

pub mod machine;
pub fn main() {
    machine::Machine::<space_invaders::SpaceInavdersInterruptHandler>::load("roms/invaders.rom")
        .expect("couldn't load rom")
        .run()
        .unwrap()
}
