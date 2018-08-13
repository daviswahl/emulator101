use disassembler;
use emulator::instructions;
use emulator::*;
use num::FromPrimitive;
use ops::*;

pub(crate) fn emulate(state: &mut State) -> Result<(), String> {
    use ops::OpCode::*;
    use ops::Register::*;
    use std::ops::Index;

    let op = OpCode::from_u8(*state.memory.index(state.pc as usize)).ok_or("unknown op code")?;

    let result = match op {
        OpCode::NOP_0
        | OpCode::NOP_1
        | OpCode::NOP_2
        | OpCode::NOP_3
        | OpCode::NOP_4
        | OpCode::NOP_5
        | OpCode::NOP_6
        | OpCode::NOP_7
        | OpCode::NOP_8
        | OpCode::NOP_9
        | OpCode::NOP_10 => Ok(()),
        ADD_A => instructions::add(A, state),
        ADD_B => instructions::add(B, state),
        ADD_C => instructions::add(C, state),
        ADD_D => instructions::add(D, state),
        ADD_E => instructions::add(E, state),
        ADD_H => instructions::add(H, state),
        ADD_L => instructions::add(L, state),
        ADD_M => instructions::add(M, state),

        ADI => instructions::adi(state),

        JMP => instructions::jmp(state),

        s => Err(format!("unimplemented op {:?}", s)),
    };

    state.pc += 1;
    result
}
