use crate::machine::cpu::disassembler::disassemble;
use crate::machine::cpu::instructions;
use crate::machine::cpu::{Error, ErrorKind};
use failure::ResultExt;
use num::FromPrimitive;
use ringbuffer::RingBufferStore;

macro_rules! simple {
    ($cpu:ident, $cycles:expr, $e:expr) => {{
        $cpu.advance()?;
        $e;
        Ok($cycles)
    }};
}

use crate::machine::cpu::ops::OpCode;
use crate::machine::CPUInterface;
use crate::machine::MachineInterface;

pub fn emulate<I: MachineInterface>(cpu: &mut CPUInterface, interface: &I) -> Result<u8, Error> {
    use crate::machine::cpu::ops::OpCode::*;
    use crate::machine::cpu::ops::Register::*;
    let code = cpu.read(cpu.cpu.pc)?;
    let op = OpCode::from_u8(code).unwrap();

    let instruction = disassemble(&cpu.memory, cpu.cpu.pc)?;
    cpu.cpu.history.push(instruction.0);
    if cpu.cpu.debug {
        println!("{:?}", instruction);
        println!("{:?}", *cpu.cpu);
    }
    let result = match op {
        NOP_0 | NOP_1 | NOP_2 | NOP_3 | NOP_4 | NOP_5 | NOP_6 | NOP_7 | NOP_8 | NOP_9 | NOP_10 => {
            cpu.advance()?;
            Ok(4)
        }
        ADD_A => instructions::add(A, cpu),
        ADD_B => instructions::add(B, cpu),
        ADD_C => instructions::add(C, cpu),
        ADD_D => instructions::add(D, cpu),
        ADD_E => instructions::add(E, cpu),
        ADD_H => instructions::add(H, cpu),
        ADD_L => instructions::add(L, cpu),
        ADD_M => instructions::add(M, cpu),

        SUB_A => instructions::sub(A, cpu),
        SUB_B => instructions::sub(B, cpu),
        SUB_C => instructions::sub(C, cpu),
        SUB_D => instructions::sub(D, cpu),
        SUB_E => instructions::sub(E, cpu),
        SUB_H => instructions::sub(H, cpu),
        SUB_L => instructions::sub(L, cpu),
        SUB_M => instructions::sub(M, cpu),

        SUI => instructions::sui(cpu),
        SBI => instructions::sbi(cpu),

        SBB_A => instructions::sbb(A, cpu),
        SBB_B => instructions::sbb(B, cpu),
        SBB_C => instructions::sbb(C, cpu),
        SBB_D => instructions::sbb(D, cpu),
        SBB_E => instructions::sbb(E, cpu),
        SBB_H => instructions::sbb(H, cpu),
        SBB_L => instructions::sbb(L, cpu),
        SBB_M => instructions::sbb(M, cpu),

        DCX_B => instructions::dcx(B, cpu),
        DCX_D => instructions::dcx(D, cpu),
        DCX_H => instructions::dcx(H, cpu),
        DCX_SP => instructions::dcx(SP, cpu),

        DAD_B => instructions::dad(B, cpu),
        DAD_D => instructions::dad(D, cpu),
        DAD_H => instructions::dad(H, cpu),
        DAD_SP => instructions::dad(SP, cpu),

        ADC_A => instructions::adc(A, cpu),
        ADC_B => instructions::adc(B, cpu),
        ADC_C => instructions::adc(C, cpu),
        ADC_D => instructions::adc(D, cpu),
        ADC_E => instructions::adc(E, cpu),
        ADC_H => instructions::adc(H, cpu),
        ADC_L => instructions::adc(L, cpu),
        ADC_M => instructions::adc(M, cpu),

        ADI => instructions::adi(cpu),

        MVI_A => instructions::mvi(A, cpu),
        MVI_B => instructions::mvi(B, cpu),
        MVI_C => instructions::mvi(C, cpu),
        MVI_D => instructions::mvi(D, cpu),
        MVI_E => instructions::mvi(E, cpu),
        MVI_H => instructions::mvi(H, cpu),
        MVI_L => instructions::mvi(L, cpu),
        MVI_M => instructions::mvi(M, cpu),

        LXI_SP => instructions::lxi(SP, cpu),
        LXI_B => instructions::lxi(B, cpu),
        LXI_D => instructions::lxi(D, cpu),
        LXI_H => instructions::lxi(H, cpu),

        LDAX_B => instructions::ldax(B, cpu),
        LDAX_D => instructions::ldax(D, cpu),

        // Mov
        MOV_A_A => instructions::mov(A, A, cpu),
        MOV_A_B => instructions::mov(A, B, cpu),
        MOV_A_C => instructions::mov(A, C, cpu),
        MOV_A_D => instructions::mov(A, D, cpu),
        MOV_A_E => instructions::mov(A, E, cpu),
        MOV_A_H => instructions::mov(A, H, cpu),
        MOV_A_L => instructions::mov(A, L, cpu),
        MOV_A_M => instructions::mov(A, M, cpu),

        MOV_B_A => instructions::mov(B, A, cpu),
        MOV_B_B => instructions::mov(B, B, cpu),
        MOV_B_C => instructions::mov(B, C, cpu),
        MOV_B_D => instructions::mov(B, D, cpu),
        MOV_B_E => instructions::mov(B, E, cpu),
        MOV_B_H => instructions::mov(B, H, cpu),
        MOV_B_L => instructions::mov(B, L, cpu),
        MOV_B_M => instructions::mov(B, M, cpu),

        MOV_C_A => instructions::mov(C, A, cpu),
        MOV_C_B => instructions::mov(C, B, cpu),
        MOV_C_C => instructions::mov(C, C, cpu),
        MOV_C_D => instructions::mov(C, D, cpu),
        MOV_C_E => instructions::mov(C, E, cpu),
        MOV_C_H => instructions::mov(C, H, cpu),
        MOV_C_L => instructions::mov(C, L, cpu),
        MOV_C_M => instructions::mov(C, M, cpu),

        MOV_D_A => instructions::mov(D, A, cpu),
        MOV_D_B => instructions::mov(D, B, cpu),
        MOV_D_C => instructions::mov(D, C, cpu),
        MOV_D_D => instructions::mov(D, D, cpu),
        MOV_D_E => instructions::mov(D, E, cpu),
        MOV_D_H => instructions::mov(D, H, cpu),
        MOV_D_L => instructions::mov(D, L, cpu),
        MOV_D_M => instructions::mov(D, M, cpu),

        MOV_E_A => instructions::mov(E, A, cpu),
        MOV_E_B => instructions::mov(E, B, cpu),
        MOV_E_C => instructions::mov(E, C, cpu),
        MOV_E_D => instructions::mov(E, D, cpu),
        MOV_E_E => instructions::mov(E, E, cpu),
        MOV_E_H => instructions::mov(E, H, cpu),
        MOV_E_L => instructions::mov(E, L, cpu),
        MOV_E_M => instructions::mov(E, M, cpu),

        MOV_H_A => instructions::mov(H, A, cpu),
        MOV_H_B => instructions::mov(H, B, cpu),
        MOV_H_C => instructions::mov(H, C, cpu),
        MOV_H_D => instructions::mov(H, D, cpu),
        MOV_H_E => instructions::mov(H, E, cpu),
        MOV_H_H => instructions::mov(H, H, cpu),
        MOV_H_L => instructions::mov(H, L, cpu),
        MOV_H_M => instructions::mov(H, M, cpu),

        MOV_L_A => instructions::mov(L, A, cpu),
        MOV_L_B => instructions::mov(L, B, cpu),
        MOV_L_C => instructions::mov(L, C, cpu),
        MOV_L_D => instructions::mov(L, D, cpu),
        MOV_L_E => instructions::mov(L, E, cpu),
        MOV_L_H => instructions::mov(L, H, cpu),
        MOV_L_L => instructions::mov(L, L, cpu),
        MOV_L_M => instructions::mov(L, M, cpu),

        MOV_M_A => instructions::mov(M, A, cpu),
        MOV_M_B => instructions::mov(M, B, cpu),
        MOV_M_C => instructions::mov(M, C, cpu),
        MOV_M_D => instructions::mov(M, D, cpu),
        MOV_M_E => instructions::mov(M, E, cpu),
        MOV_M_H => instructions::mov(M, H, cpu),
        MOV_M_L => instructions::mov(M, L, cpu),

        // ARITH
        INX_B => instructions::inx(B, cpu),
        INX_D => instructions::inx(D, cpu),
        INX_SP => instructions::inx(SP, cpu),
        INX_H => instructions::inx(H, cpu),

        INR_A => instructions::inr(A, cpu),
        INR_B => instructions::inr(B, cpu),
        INR_C => instructions::inr(C, cpu),
        INR_D => instructions::inr(D, cpu),
        INR_E => instructions::inr(E, cpu),
        INR_H => instructions::inr(H, cpu),
        INR_L => instructions::inr(L, cpu),
        INR_M => instructions::inr(M, cpu),

        DCR_A => instructions::dcr(A, cpu),
        DCR_B => instructions::dcr(B, cpu),
        DCR_C => instructions::dcr(C, cpu),
        DCR_D => instructions::dcr(D, cpu),
        DCR_E => instructions::dcr(E, cpu),
        DCR_H => instructions::dcr(H, cpu),
        DCR_L => instructions::dcr(L, cpu),
        DCR_M => instructions::dcr(M, cpu),

        ACI => instructions::aci(cpu),

        ANI => instructions::ani(cpu),

        // STACK
        PUSH_B => instructions::push(B, cpu),
        PUSH_D => instructions::push(D, cpu),
        PUSH_H => instructions::push(H, cpu),
        PUSH_PSW => instructions::push(PSW, cpu),

        // STACK
        POP_B => instructions::pop(B, cpu),
        POP_D => instructions::pop(D, cpu),
        POP_H => instructions::pop(H, cpu),
        POP_PSW => instructions::pop(PSW, cpu),

        STAX_B => instructions::stax(B, cpu),
        STAX_D => instructions::stax(D, cpu),
        STA => instructions::sta(cpu),
        LDA => instructions::lda(cpu),

        LHLD => instructions::lhld(cpu),
        SHLD => instructions::shld(cpu),
        XCHG => instructions::xchg(cpu),
        XTHL => instructions::xthl(cpu),

        CMA => simple!(cpu, 4, cpu.cpu.a = !cpu.cpu.a),

        CPI => instructions::cpi(cpu),
        CMP_A => instructions::cmp(A, cpu),
        CMP_B => instructions::cmp(B, cpu),
        CMP_C => instructions::cmp(C, cpu),
        CMP_D => instructions::cmp(D, cpu),
        CMP_E => instructions::cmp(E, cpu),
        CMP_L => instructions::cmp(L, cpu),
        CMP_H => instructions::cmp(H, cpu),
        CMP_M => instructions::cmp(M, cpu),

        RIM => {
            cpu.advance()?;
            Ok(4)
        }

        // BRANCH
        CALL => instructions::call(cpu),
        CPO => instructions::call_if(cpu, |s| !s.cpu.cc.p),
        CNZ => instructions::call_if(cpu, |s| !s.cpu.cc.z),
        CNC => instructions::call_if(cpu, |s| !s.cpu.cc.cy),
        CC => instructions::call_if(cpu, |s| s.cpu.cc.cy),
        CM => instructions::call_if(cpu, |s| s.cpu.cc.s),
        CPE => instructions::call_if(cpu, |s| s.cpu.cc.p),
        CP => instructions::call_if(cpu, |s| !s.cpu.cc.s),
        CZ => instructions::call_if(cpu, |s| s.cpu.cc.z),

        JNZ => instructions::jmp_if(cpu, |s| !s.cpu.cc.z),
        JNC => instructions::jmp_if(cpu, |s| !s.cpu.cc.cy),
        JM => instructions::jmp_if(cpu, |s| s.cpu.cc.s),
        JZ => instructions::jmp_if(cpu, |s| s.cpu.cc.z),

        JPE => instructions::jmp_if(cpu, |s| s.cpu.cc.p),
        JPO => instructions::jmp_if(cpu, |s| !s.cpu.cc.p),

        JP => instructions::jmp_if(cpu, |s| !s.cpu.cc.s),
        JC => instructions::jmp_if(cpu, |s| s.cpu.cc.cy),
        JMP => instructions::jmp_if(cpu, |_| true),

        RET => instructions::ret_if(cpu, |_| true),
        RZ => instructions::ret_if(cpu, |s| s.cpu.cc.z),
        RNZ => instructions::ret_if(cpu, |s| !s.cpu.cc.z),
        RNC => instructions::ret_if(cpu, |s| !s.cpu.cc.cy),
        RPE => instructions::ret_if(cpu, |s| s.cpu.cc.p),
        RPO => instructions::ret_if(cpu, |s| !s.cpu.cc.p),
        RP => instructions::ret_if(cpu, |s| !s.cpu.cc.s),
        RM => instructions::ret_if(cpu, |s| s.cpu.cc.s),
        RC => instructions::ret_if(cpu, |s| s.cpu.cc.cy),
        STC => simple!(cpu, 4, cpu.cpu.cc.cy = true),
        CMC => instructions::cmc(cpu),

        // LOGICAL
        XRA_A => instructions::log(A, cpu, 4, |a, b| a ^ b),
        XRA_B => instructions::log(B, cpu, 4, |a, b| a ^ b),
        XRA_C => instructions::log(C, cpu, 4, |a, b| a ^ b),
        XRA_D => instructions::log(D, cpu, 4, |a, b| a ^ b),
        XRA_E => instructions::log(E, cpu, 4, |a, b| a ^ b),
        XRA_H => instructions::log(H, cpu, 4, |a, b| a ^ b),
        XRA_L => instructions::log(L, cpu, 4, |a, b| a ^ b),
        XRA_M => instructions::log(M, cpu, 7, |a, b| a ^ b),

        ANA_A => instructions::log(A, cpu, 4, |a, b| a & b),
        ANA_B => instructions::log(B, cpu, 4, |a, b| a & b),
        ANA_C => instructions::log(C, cpu, 4, |a, b| a & b),
        ANA_D => instructions::log(D, cpu, 4, |a, b| a & b),
        ANA_E => instructions::log(E, cpu, 4, |a, b| a & b),
        ANA_H => instructions::log(H, cpu, 4, |a, b| a & b),
        ANA_L => instructions::log(L, cpu, 4, |a, b| a & b),
        ANA_M => instructions::log(M, cpu, 7, |a, b| a & b),

        ORA_A => instructions::log(A, cpu, 4, |a, b| a | b),
        ORA_B => instructions::log(B, cpu, 4, |a, b| a | b),
        ORA_C => instructions::log(C, cpu, 4, |a, b| a | b),
        ORA_D => instructions::log(D, cpu, 4, |a, b| a | b),
        ORA_E => instructions::log(E, cpu, 4, |a, b| a | b),
        ORA_H => instructions::log(H, cpu, 4, |a, b| a | b),
        ORA_L => instructions::log(L, cpu, 4, |a, b| a | b),
        ORA_M => instructions::log(M, cpu, 7, |a, b| a | b),

        ORI => instructions::logi(cpu, 7, |a, b| a | b),
        XRI => instructions::logi(cpu, 7, |a, b| a ^ b),

        RLC => instructions::rlc(cpu),
        RAL => instructions::ral(cpu),
        RRC => instructions::rrc(cpu),

        RAR => instructions::rar(cpu),

        SPHL => instructions::sphl(cpu),
        PCHL => instructions::pchl(cpu),

        IN => {
            cpu.advance()?;
            let port = cpu.read_1()?;
            interface.handle_in(cpu, port)?;
            Ok(10)
        }
        OUT => {
            cpu.advance()?;
            let port = cpu.read_1()?;
            interface.handle_out(cpu, port)?;
            Ok(10)
        }

        EI => simple!(cpu, 4, cpu.cpu.int_enable = 1),
        DI => simple!(cpu, 4, cpu.cpu.int_enable = 0),

        s => Err(ErrorKind::UnimplementedOp(s))?,
    }?;

    cpu.cpu.iters += 1;
    cpu.cpu.cycles += u128::from(result);
    Ok(result)
}
