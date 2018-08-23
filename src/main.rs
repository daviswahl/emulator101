#![feature(uniform_paths)]
#![feature(futures_api)]
#![feature(arbitrary_self_types)]
#![feature(option_replace)]
#![feature(duration_as_u128)]

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate failure;

extern crate crossbeam_channel;
extern crate ggez;

use std::any::Any;

mod diag;
mod error;
mod space_invaders;

use crate::failure::Backtrace;
use crate::failure::Context;
use crate::failure::Fail;
use error::EmulatorError;

pub mod machine;
pub fn main() -> Result<(), EmulatorError> {
    machine::Machine::load::<space_invaders::SpaceInvaders>("roms/invaders.rom")
        .expect("couldn't load rom")
        .run()
}
