use ops::Instruction;
use ops::Register;

pub(crate) struct State {
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
    pub memory: Vec<u8>,
    pub int_enable: u8,
    pub iters: u64,
    pub last_instruction: Option<(Instruction, usize)>,
    pub break_on: Option<usize>,
    pub debug: bool,
}

use std::fmt;
impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "
        a: {:X?}\tbc: {:X?}{:X?}\tdc: {:X?}{:X?}\thl: {:X?}{:X?}\tpc {:X?}\tsp: {:X?}\titers: {}
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
            self.iters,
            self.cc,
        )
    }
}

impl State {
    pub fn get_u8(&self, r: Register) -> u8 {
        match r {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            _ => unimplemented!(),
        }
    }

    pub fn set_u8(&mut self, r: Register, b: u8) {
        match r {
            Register::A => self.a = b,
            Register::B => self.b = b,
            Register::C => self.c = b,
            Register::D => self.d = b,
            Register::E => self.e = b,
            Register::H => self.h = b,
            Register::L => self.l = b,
            _ => unimplemented!(),
        }
    }

    pub fn read_1(&mut self) -> Result<u8, String> {
        let pc = self.pc as usize;
        if self.memory.len() >= pc {
            self.pc += 1;
            Ok(self.memory[pc])
        } else {
            Err(format!(
                "Tried to read out of range address: {:#X?}, len: {:#X?}",
                self.pc,
                self.memory.len()
            ))
        }
    }

    pub fn read(&self, offset: u16) -> Result<u8, String> {
        let offset = offset as usize;
        if self.memory.len() > offset {
            Ok(self.memory[offset])
        } else {
            Err(format!(
                "Tried to read out of range address: {}, len: {}",
                offset,
                self.memory.len()
            ))
        }
    }

    pub fn write(&mut self, offset: u16, data: u8) -> Result<(), String> {
        let offset = offset as usize;
        if self.memory.len() > offset {
            self.memory[offset] = data;
            Ok(())
        } else {
            Err(format!(
                "Tried to set out of range address: {}, len: {}",
                offset,
                self.memory.len()
            ))
        }
    }

    pub fn advance(&mut self) -> Result<(), String> {
        let pc = self.pc as usize;
        if self.memory.len() > pc + 1 {
            self.pc += 1;
            Ok(())
        } else {
            Err(format!("Cannot advance out of range: {}", self.pc + 1))
        }
    }
}

pub(crate) fn new_state(memory: Vec<u8>) -> State {
    State {
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
        memory,
        int_enable: 0x0,
        iters: 0,
        last_instruction: None,
        break_on: None,
        debug: false,
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct ConditionCodes {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
}
impl ConditionCodes {
    pub fn logic_flags(&mut self, a: u16) {
        self.cy = false;
        self.ac = false;
        self.z = a == 0;
        self.s = 0x80 == (a & 0x80);
        self.parity(a, 8);
    }

    pub fn parity(&mut self, x: u16, size: u16) {
        let mut p = 0;
        let mut i = 0;
        let mut x = x & ((1 << size) - 1);
        while i < size {
            if x & 0x1 == 1 {
                p += 1
            }
            x >>= 1;
            i += 1;
        }
        self.p = 0 == p & 0x1
    }

    pub fn zero(&mut self, v: u16) {
        self.z = v.trailing_zeros() >= 8
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
        self.parity(v, 8);
        self.zero(v)
    }
}
