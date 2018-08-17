pub mod disassembler;

use machine::cpu::disassembler::*;
use machine::cpu::ops::*;
mod ops;
use machine::memory::Memory;

mod emulate;
pub mod instructions;

use machine::cpu::ops::Register;

pub(crate) struct CPU {
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
    pub memory: Memory,
    pub int_enable: u8,
    pub iters: u64,
    pub last_instruction: Option<(Instruction, u16)>,
    pub break_on: Option<usize>,
    pub debug: bool,
}

use std::fmt;
impl fmt::Debug for CPU {
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

impl CPU {
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
        let pc = self.pc;
        let result = self.memory.read(pc + 1)?;
        self.pc += 1;
        Ok(result)
    }

    pub fn read(&self, offset: u16) -> Result<u8, String> {
        self.memory.read(offset)
    }

    pub fn write(&mut self, offset: u16, data: u8) -> Result<(), String> {
        self.memory.write(offset, data)
    }

    pub fn advance(&mut self) -> Result<(), String> {
        let pc = self.pc;
        if self.memory.len() > pc + 1 {
            self.pc += 1;
            Ok(())
        } else {
            Err(format!("Cannot advance out of range: {}", self.pc + 1))
        }
    }
}

pub(crate) fn new_state(memory: Memory) -> CPU {
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

#[cfg(test)]
mod tests {
    use super::*;
    use machine::cpu::emulate::*;

    fn diag() -> Result<(), String> {
        let mut buf = read_rom("roms/cpudiag.bin").unwrap();
        let mut memory = vec![0x0; 256];

        memory[0] = 0xc3;
        memory[1] = 0;
        memory[2] = 0x01;

        memory.append(&mut buf);

        memory[368] = 0x7;

        //println!("{:#X?}", memory[0x6AD]);
        memory[0x59D] = 0xc3;
        memory[0x59E] = 0xc2;
        memory[0x59F] = 0x5;

        let mut state = new_state(Memory::new(memory));
        state.debug = true;
        loop {
            match emulate::emulate(&mut state, |_| Ok(())) {
                Ok(_) => (),
                e @ Err(_) => return e,
            }
        }
    }

    fn run() {
        let memory = read_rom("roms/invaders.rom").unwrap();
        let mut state = new_state(Memory::new(memory));

        loop {
            match emulate::emulate(&mut state, |_| Ok(())) {
                Ok(()) => (),
                Err(s) => {
                    println!("{:}", s);
                    use std::process;
                    process::exit(1);
                }
            }
        }
    }
    fn run_pretty(state: &mut CPU) -> () {
        match emulate(state, |_| Ok(())) {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                ()
            }
        }
    }
    fn test_add_register(reg: Register, code: OpCode) {
        let mem: Vec<u8> = vec![code as u8];

        let reset = |mem: Vec<u8>| {
            let mut state = new_state(Memory::new(mem));
            state.a = 0;
            state.set_u8(reg, 1);
            state
        };

        let mut state = reset(mem.clone());
        run_pretty(&mut state);
        match reg {
            Register::A => assert_eq!(state.a, 2, "{:?}", reg),
            _ => assert_eq!(state.a, 1, "{:?}", reg),
        }

        let mut state = reset(mem.clone());
        state.a = 0;
        state.set_u8(reg, 0);

        run_pretty(&mut state);

        println!("{:?}", state);
        assert_eq!(state.a, 0);
        assert_eq!(state.cc.z, true);

        // check sign
        let mut state = reset(mem);

        state.a = 200;
        state.set_u8(reg, 200);
        run_pretty(&mut state);
        assert_eq!(state.a, 144);
        assert_eq!(state.cc.s, true);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_add_memory() {
        let mut mem: Vec<u8> = vec![0x0; 512];
        mem.insert(0, OpCode::ADD_M as u8);
        mem.insert(259, 200);
        let mut state = new_state(Memory::new(mem));
        state.h = 1;
        state.l = 3;
        state.a = 2;

        run_pretty(&mut state);

        assert_eq!(state.a, 202);
    }

    #[test]
    fn test_jmp() {
        let mut mem: Vec<u8> = vec![0x0; 512];
        mem.insert(0, OpCode::JMP as u8);
        mem.insert(1, 3);
        mem.insert(2, 1);
        mem.insert(259, OpCode::ADI as u8);
        mem.insert(260, 200);

        let mut state = new_state(Memory::new(mem));

        run_pretty(&mut state);
        println!("{:?}", state);
        run_pretty(&mut state);
        println!("{:?}", state);
        assert_eq!(state.a, 200);
    }

    macro_rules! rom {
        ($($v:expr),*) => {{
            let mut r = Vec::with_capacity(512);
            $(
                r.push($v as u8);
            )*
            r
        }};
    }

    #[test]
    fn test_adi() {
        let mut mem: Vec<u8> = vec![128; 0x0];
        mem.insert(0, OpCode::ADI as u8);
        mem.insert(1, 2);
        let mut state = new_state(Memory::new(mem));
        state.a = 2;
        run_pretty(&mut state);
        assert_eq!(state.a, 4);
    }

    #[test]
    fn test_rom() {
        let mut rom = rom!(OpCode::ADI);
        assert_eq!(rom.pop().unwrap(), OpCode::ADI as u8);

        let mut rom = rom!(OpCode::ADI, OpCode::ADD_A);
        assert_eq!(rom.pop().unwrap(), OpCode::ADD_A as u8);
        assert_eq!(rom.pop().unwrap(), OpCode::ADI as u8);
    }

    #[test]
    fn test_lxi() {
        let rom = rom!(OpCode::LXI_SP, 1, 2);
        let mut state = new_state(Memory::new(rom));
        run_pretty(&mut state);
        assert_eq!(state.sp, 513);
    }

    #[test]
    fn test_add() {
        test_add_register(Register::A, OpCode::ADD_A);
        test_add_register(Register::B, OpCode::ADD_B);
        test_add_register(Register::C, OpCode::ADD_C);
        test_add_register(Register::D, OpCode::ADD_D);
        test_add_register(Register::E, OpCode::ADD_E);
        test_add_register(Register::H, OpCode::ADD_H);
        test_add_register(Register::L, OpCode::ADD_L);
    }

    #[test]
    fn test_diag() {
        assert_eq!(diag(), Err("exit".to_string()))
    }
}
