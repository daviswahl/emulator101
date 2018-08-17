#![feature(cell_update)]

extern crate num;
extern crate num_traits;
#[macro_use]
extern crate num_derive;

pub mod machine;
pub fn main() {
    machine::Machine::with_rom("roms/invaders2.rom")
        .unwrap()
        .run()
        .unwrap()
}
macro_rules! read_1 {
    ($inst:expr) => {
        Ok(($inst, 0))
    };
    ($inst:expr, $reg:expr) => {
        Ok(($inst($reg), 0))
    };
}
macro_rules! read_2 {
    ($inst:path, $iter:ident, $pos:ident) => {
        Ok(($inst($iter.read($pos + 1)?), 1))
    };
    ($inst:path, $iter:ident, $pos:ident, $reg:expr) => {
        Ok(($inst($reg, $iter.read($pos + 1)?), 2))
    };

    ($inst:path, $iter:ident,$pos:ident, $reg:expr, $reg2:expr) => {
        Ok(($inst($reg, $reg2, $iter.read($pos + 1)?), 2))
    };
}

macro_rules! read_3 {
    ($inst:path, $iter:ident, $pos:ident) => {
        Ok(($inst($iter.read($pos + 1)?, $iter.read($pos + 2)?), 2))
    };
    ($inst:path, $iter:ident,$pos:ident, $reg:expr) => {
        Ok(($inst($reg, $iter.read($pos + 1)?, $iter.read($pos + 2)?), 2))
    };
    ($inst:path, $iter:ident,$pos:ident, $reg:expr, $reg2:expr) => {
        Ok((
            $inst($reg, $reg2, $iter.read($pos + 1)?, $iter.read($pos + 2)?),
            2,
        ))
    };
}
