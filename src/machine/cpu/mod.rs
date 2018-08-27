pub mod disassembler;

use crate::machine::cpu::ops::*;
pub mod ops;
use crate::machine::memory::Memory;

mod emulate;
mod error;
pub mod instructions;

pub use crate::machine::cpu::error::{Error, ErrorKind};

pub use crate::machine::cpu::emulate::emulate;
use crate::machine::cpu::ops::Register;
use crate::ring_buffers;
use crate::ring_buffers::RingBuffer;
use std::fmt;

pub type History = ring_buffers::RingBuffer<[Option<Instruction>; 256]>;
#[derive(Debug)]
pub struct CPU {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub cc: ConditionCodes,
    pub int_enable: u8,
    pub iters: u64,
    pub last_instruction: Option<(Instruction, u16)>,
    pub pause: bool,
    pub debug: bool,
    pub cycles: u128,
    pub history: History,
}

impl fmt::Display for History {
    fn fmt(&self, f: &'_ mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for i in self.iter() {
            write!(f, "{:?}\n", i)?;
        }
        Ok(())
    }
}
impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "
        a: {:X?}\tbc: {:X?}{:X?}\tde: {:X?}{:X?}\thl: {:X?}{:X?}\tpc {:X?}\tsp: {:X?}\tcycles: {}\titers: {}
        {:?}
        ",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.pc,
            self.sp,
            self.cycles,
            self.iters,
            self.cc,
        )
    }
}

pub struct CPUInterface<'a> {
    pub cpu: &'a mut CPU,
    pub memory: &'a mut Memory,
}

impl<'a> fmt::Display for CPUInterface<'a> {
    fn fmt(&self, f: &'_ mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.cpu.fmt(f)
    }
}

impl<'a> CPUInterface<'a> {
    pub fn get_u8(&self, r: Register) -> u8 {
        match r {
            Register::A => self.cpu.a,
            Register::B => self.cpu.b,
            Register::C => self.cpu.c,
            Register::D => self.cpu.d,
            Register::E => self.cpu.e,
            Register::H => self.cpu.h,
            Register::L => self.cpu.l,
            _ => unimplemented!(),
        }
    }

    pub fn set_u8(&mut self, r: Register, b: u8) {
        match r {
            Register::A => self.cpu.a = b,
            Register::B => self.cpu.b = b,
            Register::C => self.cpu.c = b,
            Register::D => self.cpu.d = b,
            Register::E => self.cpu.e = b,
            Register::H => self.cpu.h = b,
            Register::L => self.cpu.l = b,
            _ => unimplemented!(),
        }
    }

    pub fn read_1(&mut self) -> Result<u8, Error> {
        let result = self.read(self.cpu.pc);
        self.advance()?;
        result
    }

    pub fn read(&self, offset: u16) -> Result<u8, Error> {
        Ok(self.memory.read(offset)?)
    }

    pub fn write(&mut self, offset: u16, data: u8) -> Result<(), Error> {
        self.memory.write(offset, data)?;
        Ok(())
    }

    pub fn advance(&mut self) -> Result<(), Error> {
        let pc = (self.cpu.pc + 1) % 0x4000;
        if self.memory.len() >= pc {
            self.cpu.pc = pc;
            Ok(())
        } else {
            Err(ErrorKind::PCOutOfRange(pc, self.memory.len()).into())
        }
    }

    pub fn interrupt(&mut self, interrupt_num: u16) -> Result<(), Error> {
        self.cpu.int_enable = 0;
        let sp = self.cpu.sp;
        let low = (self.cpu.pc & 0xff) as u8;
        let high = ((self.cpu.pc & 0xFF00) >> 8) as u8;

        self.write(sp.wrapping_sub(2), low)?;
        self.write(sp.wrapping_sub(1), high)?;
        self.cpu.sp = sp.wrapping_sub(2);

        self.cpu.pc = interrupt_num.wrapping_mul(8);

        Ok(())
    }
}

pub(crate) fn new() -> CPU {
    CPU {
        a: 0x0,
        b: 0x0,
        c: 0x0,
        d: 0x0,
        e: 0x0,
        h: 0x0,
        l: 0x0,
        cc: ConditionCodes {
            z: false,
            s: false,
            p: false,
            cy: false,
            ac: false,
        },
        sp: 0x0,
        pc: 0x0,
        int_enable: 0,
        iters: 0,
        last_instruction: None,
        pause: false,
        debug: false,
        cycles: 0,
        history: ring_buffers::new([None; 256]),
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ConditionCodes {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
}
impl ConditionCodes {
    pub fn logic_flags(&mut self, a: u8) {
        self.cy = false;
        self.ac = false;
        self.z = a == 0;
        self.s = 0x80 == (a & 0x80);
        self.parity(a.into(), 8);
    }

    pub fn parity(&mut self, x: u8, size: u32) {
        self.p = PARITY_LUT[x as usize]
    }

    pub fn zero(&mut self, v: u16) {
        self.z = v & 0xff == 0
    }

    pub fn sign(&mut self, v: u16) {
        self.s = 0x80 == (v & 0x80)
    }

    pub fn carry(&mut self, v: u16) {
        self.cy = v > 0xff
    }

    pub fn arith_flags(&mut self, v: u16) {
        self.carry(v);
        self.sign(v);
        self.parity((v & 0xff) as u8, 8);
        self.zero(v)
    }

    pub fn flags_zsp(&mut self, v: u8) {
        self.z = (v == 0);
        self.s = (0x80 == (v & 0x80));
        self.parity(v, 8);
    }
}

pub fn pause<D: fmt::Debug>(debug: D) {
    use std::io;
    use std::io::Read;
    use std::io::Write;
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    writeln!(stdout, "{:?}\nPress any key to continue...", debug).unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

lazy_static! {
    static ref PARITY_LUT: [bool; 256] = {
        let mut lut = [false; 256];
        for i in 0..256 {
            lut[i] = parity(i as u8);
        }
        lut
    };
}

fn parity(x: u8) -> bool {
    let mut bit = 0;
    let mut num_bits = 0;
    let mut x = x;
    while x > 0 {
        let bitmask = 1 << bit;
        bit += 1;
        if x & bitmask > 0 {
            num_bits += 1;
        }
        x &= !bitmask;
    }
    num_bits % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parity() {
        assert!(!parity(118));
        assert!(!parity(247));
    }
}
