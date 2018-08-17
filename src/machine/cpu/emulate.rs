use machine::cpu::disassembler::disassemble;
use machine::cpu::instructions;
use machine::cpu::CPU;
use num::FromPrimitive;

macro_rules! simple {
    ($state:ident, $e:expr) => {{
        $state.advance()?;
        Ok($e)
    }};
}
use machine::cpu::ops::OpCode;

#[derive(Debug)]
pub(crate) struct Interrupt<'a> {
    code: OpCode,
    state: &'a mut CPU,
    d: u8,
}

pub(crate) fn emulate<F>(state: &mut CPU, interrupt: F) -> Result<(), String>
where
    F: Fn(Interrupt) -> Result<(), String>,
{
    use machine::cpu::ops::OpCode::*;
    use machine::cpu::ops::Register::*;
    let code = state.read(state.pc)?;
    let op = OpCode::from_u8(code).ok_or("unknown op code")?;

    //    state.last_instruction = Some(disassemble(state.memory.clone(), state.pc)?);
    //
    //    if let Some(inst) = state.last_instruction {
    //        println!("{:#X?}, {:?}", state.pc, inst.0);
    //    }
    let result = match op {
        NOP_0 | NOP_1 | NOP_2 | NOP_3 | NOP_4 | NOP_5 | NOP_6 | NOP_7 | NOP_8 | NOP_9 | NOP_10 => {
            state.advance()
        }
        ADD_A => instructions::add(A, state),
        ADD_B => instructions::add(B, state),
        ADD_C => instructions::add(C, state),
        ADD_D => instructions::add(D, state),
        ADD_E => instructions::add(E, state),
        ADD_H => instructions::add(H, state),
        ADD_L => instructions::add(L, state),
        ADD_M => instructions::add(M, state),

        SUB_A => instructions::sub(A, state),
        SUB_B => instructions::sub(B, state),
        SUB_C => instructions::sub(C, state),
        SUB_D => instructions::sub(D, state),
        SUB_E => instructions::sub(E, state),
        SUB_H => instructions::sub(H, state),
        SUB_L => instructions::sub(L, state),
        SUB_M => instructions::sub(M, state),

        SUI => instructions::sui(state),
        SBI => instructions::sbi(state),

        SBB_A => instructions::sbb(A, state),
        SBB_B => instructions::sbb(B, state),
        SBB_C => instructions::sbb(C, state),
        SBB_D => instructions::sbb(D, state),
        SBB_E => instructions::sbb(E, state),
        SBB_H => instructions::sbb(H, state),
        SBB_L => instructions::sbb(L, state),
        SBB_M => instructions::sbb(M, state),

        DCX_B => instructions::dcx(B, state),
        DCX_D => instructions::dcx(D, state),
        DCX_H => instructions::dcx(H, state),
        DCX_SP => instructions::dcx(SP, state),

        DAD_B => instructions::dad(B, state),
        DAD_D => instructions::dad(D, state),
        DAD_H => instructions::dad(H, state),
        DAD_SP => instructions::dad(SP, state),

        ADC_A => instructions::adc(A, state),
        ADC_B => instructions::adc(B, state),
        ADC_C => instructions::adc(C, state),
        ADC_D => instructions::adc(D, state),
        ADC_E => instructions::adc(E, state),
        ADC_H => instructions::adc(H, state),
        ADC_L => instructions::adc(L, state),
        ADC_M => instructions::adc(M, state),

        ADI => instructions::adi(state),

        MVI_A => instructions::mvi(A, state),
        MVI_B => instructions::mvi(B, state),
        MVI_C => instructions::mvi(C, state),
        MVI_D => instructions::mvi(D, state),
        MVI_E => instructions::mvi(E, state),
        MVI_H => instructions::mvi(H, state),
        MVI_L => instructions::mvi(L, state),
        MVI_M => instructions::mvi(M, state),

        LXI_SP => instructions::lxi(SP, state),
        LXI_B => instructions::lxi(B, state),
        LXI_D => instructions::lxi(D, state),
        LXI_H => instructions::lxi(H, state),

        LDAX_B => instructions::ldax(B, state),
        LDAX_D => instructions::ldax(D, state),

        // Mov
        MOV_A_A => instructions::mov(A, A, state),
        MOV_A_B => instructions::mov(A, B, state),
        MOV_A_C => instructions::mov(A, C, state),
        MOV_A_D => instructions::mov(A, D, state),
        MOV_A_E => instructions::mov(A, E, state),
        MOV_A_H => instructions::mov(A, H, state),
        MOV_A_L => instructions::mov(A, L, state),
        MOV_A_M => instructions::mov(A, M, state),

        MOV_B_A => instructions::mov(B, A, state),
        MOV_B_B => instructions::mov(B, B, state),
        MOV_B_C => instructions::mov(B, C, state),
        MOV_B_D => instructions::mov(B, D, state),
        MOV_B_E => instructions::mov(B, E, state),
        MOV_B_H => instructions::mov(B, H, state),
        MOV_B_L => instructions::mov(B, L, state),
        MOV_B_M => instructions::mov(B, M, state),

        MOV_C_A => instructions::mov(C, A, state),
        MOV_C_B => instructions::mov(C, B, state),
        MOV_C_C => instructions::mov(C, C, state),
        MOV_C_D => instructions::mov(C, D, state),
        MOV_C_E => instructions::mov(C, E, state),
        MOV_C_H => instructions::mov(C, H, state),
        MOV_C_L => instructions::mov(C, L, state),
        MOV_C_M => instructions::mov(C, M, state),

        MOV_D_A => instructions::mov(D, A, state),
        MOV_D_B => instructions::mov(D, B, state),
        MOV_D_C => instructions::mov(D, C, state),
        MOV_D_D => instructions::mov(D, D, state),
        MOV_D_E => instructions::mov(D, E, state),
        MOV_D_H => instructions::mov(D, H, state),
        MOV_D_L => instructions::mov(D, L, state),
        MOV_D_M => instructions::mov(D, M, state),

        MOV_E_A => instructions::mov(E, A, state),
        MOV_E_B => instructions::mov(E, B, state),
        MOV_E_C => instructions::mov(E, C, state),
        MOV_E_D => instructions::mov(E, D, state),
        MOV_E_E => instructions::mov(E, E, state),
        MOV_E_H => instructions::mov(E, H, state),
        MOV_E_L => instructions::mov(E, L, state),
        MOV_E_M => instructions::mov(E, M, state),

        MOV_H_A => instructions::mov(H, A, state),
        MOV_H_B => instructions::mov(H, B, state),
        MOV_H_C => instructions::mov(H, C, state),
        MOV_H_D => instructions::mov(H, D, state),
        MOV_H_E => instructions::mov(H, E, state),
        MOV_H_H => instructions::mov(H, H, state),
        MOV_H_L => instructions::mov(H, L, state),
        MOV_H_M => instructions::mov(H, M, state),

        MOV_L_A => instructions::mov(L, A, state),
        MOV_L_B => instructions::mov(L, B, state),
        MOV_L_C => instructions::mov(L, C, state),
        MOV_L_D => instructions::mov(L, D, state),
        MOV_L_E => instructions::mov(L, E, state),
        MOV_L_H => instructions::mov(L, H, state),
        MOV_L_L => instructions::mov(L, L, state),
        MOV_L_M => instructions::mov(L, M, state),

        MOV_M_A => instructions::mov(M, A, state),
        MOV_M_B => instructions::mov(M, B, state),
        MOV_M_C => instructions::mov(M, C, state),
        MOV_M_D => instructions::mov(M, D, state),
        MOV_M_E => instructions::mov(M, E, state),
        MOV_M_H => instructions::mov(M, H, state),
        MOV_M_L => instructions::mov(M, L, state),

        // ARITH
        INX_B => instructions::inx(B, state),
        INX_D => instructions::inx(D, state),
        INX_SP => instructions::inx(SP, state),
        INX_H => instructions::inx(H, state),

        INR_A => instructions::inr(A, state),
        INR_B => instructions::inr(B, state),
        INR_C => instructions::inr(C, state),
        INR_D => instructions::inr(D, state),
        INR_E => instructions::inr(E, state),
        INR_H => instructions::inr(H, state),
        INR_L => instructions::inr(L, state),
        INR_M => instructions::inr(M, state),

        DCR_A => instructions::dcr(A, state),
        DCR_B => instructions::dcr(B, state),
        DCR_C => instructions::dcr(C, state),
        DCR_D => instructions::dcr(D, state),
        DCR_E => instructions::dcr(E, state),
        DCR_H => instructions::dcr(H, state),
        DCR_L => instructions::dcr(L, state),
        DCR_M => instructions::dcr(M, state),

        ACI => instructions::aci(state),

        ANI => instructions::ani(state),

        // STACK
        PUSH_B => instructions::push(B, state),
        PUSH_D => instructions::push(D, state),
        PUSH_H => instructions::push(H, state),
        PUSH_PSW => instructions::push(PSW, state),

        // STACK
        POP_B => instructions::pop(B, state),
        POP_D => instructions::pop(D, state),
        POP_H => instructions::pop(H, state),
        POP_PSW => instructions::pop(PSW, state),

        STAX_B => instructions::stax(B, state),
        STAX_D => instructions::stax(D, state),
        STA => instructions::sta(state),
        LDA => instructions::lda(state),

        LHLD => instructions::lhld(state),
        SHLD => instructions::shld(state),
        XCHG => instructions::xchg(state),
        XTHL => instructions::xthl(state),

        CMA => simple!(state, state.a = !state.a),

        CPI => instructions::cpi(state),
        CMP_A => instructions::cmp(A, state),
        CMP_B => instructions::cmp(B, state),
        CMP_C => instructions::cmp(C, state),
        CMP_D => instructions::cmp(D, state),
        CMP_E => instructions::cmp(E, state),
        CMP_L => instructions::cmp(L, state),
        CMP_H => instructions::cmp(H, state),
        CMP_M => instructions::cmp(M, state),

        // BRANCH
        CALL => instructions::call(state),
        CPO => instructions::call_if(state, |s| !s.cc.p),
        CNZ => instructions::call_if(state, |s| !s.cc.z),
        CNC => instructions::call_if(state, |s| !s.cc.cy),
        CC => instructions::call_if(state, |s| s.cc.cy),
        CM => instructions::call_if(state, |s| s.cc.s),
        CPE => instructions::call_if(state, |s| s.cc.p),
        CP => instructions::call_if(state, |s| !s.cc.s),
        CZ => instructions::call_if(state, |s| s.cc.z),

        JNZ => instructions::jmp_if(state, |s| !s.cc.z),
        JNC => instructions::jmp_if(state, |s| !s.cc.cy),
        JM => instructions::jmp_if(state, |s| s.cc.s),
        JZ => instructions::jmp_if(state, |s| s.cc.z),

        JPE => instructions::jmp_if(state, |s| s.cc.p),
        JPO => instructions::jmp_if(state, |s| !s.cc.p),

        JP => instructions::jmp_if(state, |s| !s.cc.s),
        JC => instructions::jmp_if(state, |s| s.cc.cy),
        JMP => instructions::jmp_if(state, |_| true),

        RET => instructions::ret_if(state, |_| true),
        RZ => instructions::ret_if(state, |s| s.cc.z),
        RNZ => instructions::ret_if(state, |s| !s.cc.z),
        RNC => instructions::ret_if(state, |s| !s.cc.cy),
        RPE => instructions::ret_if(state, |s| s.cc.p),
        RPO => instructions::ret_if(state, |s| !s.cc.p),
        RP => instructions::ret_if(state, |s| !s.cc.s),
        RM => instructions::ret_if(state, |s| s.cc.s),
        RC => instructions::ret_if(state, |s| s.cc.cy),
        STC => simple!(state, state.cc.cy = true),
        CMC => instructions::cmc(state),

        // LOGICAL
        XRA_A => instructions::log(A, state, |a, b| a ^ b),
        XRA_B => instructions::log(B, state, |a, b| a ^ b),
        XRA_C => instructions::log(C, state, |a, b| a ^ b),
        XRA_D => instructions::log(D, state, |a, b| a ^ b),
        XRA_E => instructions::log(E, state, |a, b| a ^ b),
        XRA_H => instructions::log(H, state, |a, b| a ^ b),
        XRA_L => instructions::log(L, state, |a, b| a ^ b),
        XRA_M => instructions::log(M, state, |a, b| a ^ b),

        ANA_A => instructions::log(A, state, |a, b| a & b),
        ANA_B => instructions::log(B, state, |a, b| a & b),
        ANA_C => instructions::log(C, state, |a, b| a & b),
        ANA_D => instructions::log(D, state, |a, b| a & b),
        ANA_E => instructions::log(E, state, |a, b| a & b),
        ANA_H => instructions::log(H, state, |a, b| a & b),
        ANA_L => instructions::log(L, state, |a, b| a & b),
        ANA_M => instructions::log(M, state, |a, b| a & b),

        ORA_A => instructions::log(A, state, |a, b| a | b),
        ORA_B => instructions::log(B, state, |a, b| a | b),
        ORA_C => instructions::log(C, state, |a, b| a | b),
        ORA_D => instructions::log(D, state, |a, b| a | b),
        ORA_E => instructions::log(E, state, |a, b| a | b),
        ORA_H => instructions::log(H, state, |a, b| a | b),
        ORA_L => instructions::log(L, state, |a, b| a | b),
        ORA_M => instructions::log(M, state, |a, b| a | b),

        ORI => instructions::logi(state, |a, b| a | b),
        XRI => instructions::logi(state, |a, b| a ^ b),

        RLC => instructions::rlc(state),
        RAL => instructions::ral(state),
        RRC => instructions::rrc(state),

        RAR => instructions::rar(state),

        SPHL => instructions::sphl(state),
        PCHL => instructions::pchl(state),

        IN | OUT => {
            state.advance()?;
            let port = state.read_1()?;
            interrupt(Interrupt {
                d: port,
                code: op,
                state,
            })
        }

        EI => simple!(state, state.int_enable = 1),
        DI => simple!(state, state.int_enable = 0),

        s => Err(format!("unimplemented op {:?}", s)),
    };

    state.iters += 1;

    println!("{:?}", state);
    if state.iters >= 37410 {
        //        println!(
        //            "{}",
        //            disassemble_range(&state.memory, state.pc as usize..(state.pc + 10) as usize)?
        //        );
        // pause();
    }
    result.map_err(|e| e)
}

pub fn pause() {
    use std::io;
    use std::io::Read;
    use std::io::Write;
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    writeln!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
