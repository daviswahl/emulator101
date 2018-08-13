use ops::Register;
use emulator::State;
pub (crate) fn add(reg: Register, state: &mut State) -> Result<(), String> {
    use std::ops::Index;
    let answer = match reg {
        Register::M => {
            let offset = ((state.h as u16) << 8) | state.l as u16;
            (state.a as u16) + (*state.memory.index(offset as usize) as u16)
        }
        r => (state.a as u16) + (state.get_u8(r) as u16),
    };
    state.cc.zero(answer);
    state.cc.parity(answer);
    state.cc.sign(answer);
    state.cc.carry(answer);
    state.a = (answer & 0xff) as u8;
    Ok(())
}

pub(crate) fn adi(state: &mut State) -> Result<(), String> {
    let val = state.read_1()?;
    let answer: u16 = (state.a as u16) + (val as u16);

    state.cc.zero(answer);
    state.cc.parity(answer);
    state.cc.sign(answer);
    state.cc.carry(answer);
    state.a = (answer & 0xff) as u8;
    Ok(())
}

pub (crate) fn jmp(state: &mut State) -> Result<(), String> {
    let h = state.read_1()?;
    let l = state.read_1()?;
    let offset = (h as u16)<<8 | l as u16;
    state.pc = offset;

    Ok(())
}
