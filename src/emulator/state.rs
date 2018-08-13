use ops::Register;
use std::ops::Index;
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

    pub fn reset(self) -> State {
        new_state(self.memory)
    }

    pub fn read_1(&mut self) -> Result<u8, String> {
        self.pc += 1;
        if (self.memory.len() >= self.pc as usize) {
            Ok(*self.memory.index(self.pc as usize))
        } else {
            Err(format!("Tried to read out of range address: {}", self.pc))
        }
    }

    pub fn read_m(&mut self, offset: usize) -> Result<u8, String> {
        if (self.memory.len() >= offset) {
            Ok(*self.memory.index(offset))
        } else {
            Err(format!("Tried to read out of range address: {}", offset))
        }
    }
}

pub (crate) fn new_state(memory: Vec<u8>) -> State {
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
    }
}

#[derive(Copy, Clone)]
pub (crate) struct ConditionCodes {
    pub z: bool,
    pub s: bool,
    pub p: bool,
    pub cy: bool,
    pub ac: bool,
}
impl ConditionCodes {
    pub fn parity(&mut self, v: u16) {
        if (v & 0xff) as u8 % 2 == 1 {
            self.p = false;
        } else {
            self.p = true;
        }
    }

    pub fn zero(&mut self, v: u16) {
        if v & 0xff == 0 {
            self.z = true;
        } else {
            self.z = false;
        }
    }

    pub fn sign(&mut self, v: u16) {
        if v & 0x80 != 0 {
            self.s = true;
        } else {
            self.s = false;
        }
    }

    pub fn carry(&mut self, v: u16) {
        if v > 0xff {
            self.cy = true;
        } else {
            self.cy = false;
        }
    }
}
