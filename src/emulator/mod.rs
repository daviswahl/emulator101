use disassembler;
use disassembler::*;
use ops::*;
use std::ops::Index;

mod state;
use self::state::*;
mod emulate;
use emulator::emulate::emulate;
pub mod instructions;


pub fn run() {
    let memory = read_rom("roms/invaders.rom").unwrap();
    let mut state = state::new_state(memory);

    loop {
        match emulate::emulate(&mut state) { 
            Ok(()) => (),
            Err(s) => panic!(s)
        }
    } 
}

use std::collections::HashMap;
pub type MMap = HashMap<u16, Instruction>;

#[cfg(test)]
mod tests {
    use super::*;
    fn test_add_register(reg: Register, code: OpCode) {
        let mem: Vec<u8> = vec![code as u8];
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
    fn test_add_memory() {
        let mut mem: Vec<u8> = vec![0x0; 512];
        mem.insert(0, OpCode::ADD_M as u8);
        mem.insert(259, 200);
        let mut state = new_state(mem);
        state.h = 1;
        state.l = 3;
        state.a = 2;

        emulate(&mut state).unwrap();

        assert_eq!(state.a, 202);
    }

    #[test]
    fn test_jmp() {
        let mut mem: Vec<u8> = vec![0x0;512];
        mem.insert(0, OpCode::JMP as u8);
        mem.insert(1, 1);
        mem.insert(2, 3);
        mem.insert(260, OpCode::ADI as u8);
        mem.insert(261, 200);

        let mut state = new_state(mem);

        emulate(&mut state).unwrap();
        emulate(&mut state).unwrap();
        assert_eq!(state.a, 200);
    }

    #[test]
    fn test_adi() {
        let mut mem: Vec<u8> = vec![128; 0x0];
        mem.insert(0, OpCode::ADI as u8);
        mem.insert(1, 2);
        let mut state = new_state(mem);
        state.a = 2;
        emulate(&mut state).unwrap();
        assert_eq!(state.a, 4);
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
}
