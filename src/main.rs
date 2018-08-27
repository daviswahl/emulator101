#![feature(uniform_paths)]
#![feature(futures_api)]
#![feature(arbitrary_self_types)]
#![feature(option_replace)]
#![feature(duration_as_u128)]
#![feature(const_fn)]
#![feature(const_let)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate num_derive;

extern crate num;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate ringbuffer;
extern crate core;
extern crate crossbeam_channel;
extern crate ggez;

mod diag;
mod space_invaders;

use crate::failure::Fail;

mod ring_buffers {
    impl_ring_buffer!(256, 4);
}

pub mod machine;
pub fn main() -> Result<(), machine::Error> {
    machine::Machine::load::<space_invaders::SpaceInvaders>("roms/invaders.rom")
        .expect("couldn't load rom")
        .run()?;
    Ok(())
}
