use disassembler::*;
use ops::*;

mod state;
use self::state::*;
mod emulate;
pub mod instructions;

pub fn diag() -> Result<(), String> {
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

    let mut state = state::new_state(memory);
    state.debug = true;
    loop {
        match emulate::emulate(&mut state, |_| Ok(())) {
            Ok(_) => (),
            e @ Err(_) => return e,
        }
    }
}

pub fn run() {
    let memory = read_rom("roms/invaders.rom").unwrap();
    let mut state = state::new_state(memory);

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

use std::collections::HashMap;
pub type MMap = HashMap<u16, Instruction>;

#[cfg(test)]
mod tests {
    use super::*;
    use emulator::emulate::emulate;

    fn run_pretty(state: &mut State) -> () {
        match emulate(state) {
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
            let mut state = new_state(mem);
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
        let mut state = new_state(mem);
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

        let mut state = new_state(mem);

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
        let mut state = new_state(mem);
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
        let mut state = new_state(rom);
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
}
