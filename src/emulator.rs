use disassembler;
use disassembler::*;
use ops::*;

#[derive(Copy, Clone)]
struct ConditionCodes {
    z: bool,
    s: bool,
    p: bool,
    cy: bool,
    ac: bool,
}
impl ConditionCodes {
    fn parity(&mut self, v: u16) {
        if (v & 0xff) as u8 % 2 == 1 {
            self.p = false;
        } else {
            self.p = true;
        }
    }

    fn zero(&mut self, v: u16) {
        if v & 0xff == 0 {
            self.z = true;
        } else {
            self.z = false;
        }
    }

    fn sign(&mut self, v: u16) {
        if v & 0x80 != 0 {
            self.s = true;
        } else {
            self.s = false;
        }
    }

    fn carry(&mut self, v: u16) {
        if v > 0xff {
            self.cy = true;
        } else {
            self.cy = false;
        }
    }
}
struct State {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    cc: ConditionCodes,
    memory: MMap,
    int_enable: u8,
}

impl State {
    fn get_u8(&self, r: Register) -> u8 {
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

    fn set_u8(&mut self, r: Register, b: u8) {
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

    fn reset(self) -> State {
        new_state(self.memory)
    }
}

fn new_state(memory: MMap) -> State {
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
        memory: memory,
        int_enable: 0x0,
    }
}

fn add(reg: Register, state: &mut State) {
    let answer: u16 = (state.a as u16) + (state.get_u8(reg) as u16);

    state.cc.zero(answer);
    state.cc.parity(answer);
    state.cc.sign(answer);
    state.cc.carry(answer);
    state.a = (answer & 0xff) as u8
}

fn emulate(state: &mut State) -> Result<(), &'static str> {
    use ops::Instruction::*;

    let instruction = match state.memory.get(&state.pc) {
        Some(inst) => inst,
        None => return Err("bad inst"),
    }.clone();

    let result = match instruction {
        NOP => Ok(()),
        ADD(reg) => Ok(add(reg, state)),
        inst => unimplemented!("{:?}", inst),
    };

    state.pc += 1;
    result
}

pub fn run() {
    let memory = mmap_rom();
    let mut state = new_state(memory);

    while let Ok(()) = emulate(&mut state) {}
}

use std::collections::HashMap;
pub type MMap = HashMap<u16, Instruction>;

pub fn mmap<I: Iterator<Item = u8>>(reader: &mut OpReader<I>) -> MMap {
    let mut map = HashMap::new();
    while let Some(Ok((inst, pc))) = reader.next() {
        map.insert(pc, inst);
    }
    map
}

pub fn mmap_rom() -> MMap {
    let rom = read_rom("roms/invaders.rom").unwrap();
    let mut r = reader(rom);
    mmap(&mut r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_8bit_registers() -> Vec<Register> {
        vec![
            Register::A,
            Register::B,
            Register::C,
            Register::D,
            Register::E,
            Register::H,
            Register::L,
        ]
    }

    fn test_add_register(reg: Register) {
        let mut mem: MMap = HashMap::new();
        mem.insert(0x0, Instruction::ADD(reg));
        let mut state = new_state(mem);
        state.a = 0;
        state.set_u8(reg, 1);
        emulate(&mut state).unwrap();
        match reg {
            Register::A => assert_eq!(state.a, 2, "{:?}", reg),
            _ => assert_eq!(state.a, 1, "{:?}", reg),
        }

        let mut state = state.reset();

        emulate(&mut state).unwrap();

        assert_eq!(state.a, 0);
        assert_eq!(state.cc.z, true);

        // check sign
        let mut state = state.reset();

        state.a = 200;
        state.set_u8(reg, 200);
        emulate(&mut state).unwrap();
        assert_eq!(state.a, 144);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_add() {
        for reg in all_8bit_registers() {
            test_add_register(reg)
        }
    }
}
