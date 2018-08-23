#![feature(uniform_paths)]
#![feature(futures_api)]
#![feature(arbitrary_self_types)]
#![feature(option_replace)]
#![feature(duration_as_u128)]

#[macro_use]
extern crate num_derive;

extern crate crossbeam_channel;
extern crate ggez;

// use futures::future::Future;

mod diag;
mod space_invaders;

pub mod machine;
pub fn main() -> Result<(), String> {
    machine::Machine::load::<space_invaders::SpaceInvaders>("roms/invaders.rom")
        .expect("couldn't load rom")
        .run()
}
