use machine::cpu::ops::*;

use num::FromPrimitive;
use std::fs;
use std::path::Path;

use machine::memory::Memory;
use std::ops::Index;
use std::ops::Range;

pub fn read_rom(p: &'static str) -> Result<Vec<u8>, &'static str> {
    let mut v = fs::read(Path::new(p)).map_err(|_| "failed to read file")?;
    v.extend(vec![0x0; 8192]);
    Ok(v)
}

pub struct OpReader {
    buf: Memory,
    pc: u16,
}

macro_rules! read_1 {
    ($inst:expr) => {
        Ok(($inst, 0))
    };
    ($inst:expr, $reg:expr) => {
        Ok(($inst($reg), 0))
    };
}
macro_rules! read_2 {
    ($inst:path, $iter:ident, $pos:ident) => {
        Ok(($inst($iter.read($pos + 1)?), 1))
    };
    ($inst:path, $iter:ident, $pos:ident, $reg:expr) => {
        Ok(($inst($reg, $iter.read($pos + 1)?), 2))
    };

    ($inst:path, $iter:ident,$pos:ident, $reg:expr, $reg2:expr) => {
        Ok(($inst($reg, $reg2, $iter.read($pos + 1)?), 2))
    };
}

macro_rules! read_3 {
    ($inst:path, $iter:ident, $pos:ident) => {
        Ok(($inst($iter.read($pos + 1)?, $iter.read($pos + 2)?), 2))
    };
    ($inst:path, $iter:ident,$pos:ident, $reg:expr) => {
        Ok(($inst($reg, $iter.read($pos + 1)?, $iter.read($pos + 2)?), 2))
    };
    ($inst:path, $iter:ident,$pos:ident, $reg:expr, $reg2:expr) => {
        Ok((
            $inst($reg, $reg2, $iter.read($pos + 1)?, $iter.read($pos + 2)?),
            2,
        ))
    };
}

impl OpReader {
    fn read_all(&mut self) -> Vec<(u16, Instruction)> {
        let mut buf = vec![];
        loop {
            let pos = self.pc;
            self.pc += 1;
            if self.buf.len() > pos {
                if let Ok((ins, size)) = disassemble(self.buf.clone(), pos) {
                    self.pc += size;
                    buf.push((pos, ins));
                }
            } else {
                break;
            }
        }
        buf
    }
}

pub(crate) fn disassemble_range(buf: Memory, r: Range<u16>) -> Result<String, String> {
    let mut s = String::new();
    for i in r {
        s.push_str(format!("{:?}\n", disassemble(buf.clone(), i)?.0).as_ref());
    }
    Ok(s)
}

pub(crate) fn disassemble(buf: Memory, pos: u16) -> Result<(Instruction, u16), String> {
    use machine::cpu::ops::Register::*;
    let code = OpCode::from_u8(buf.read(pos as u16)?).ok_or("out of range")?;
    match code {
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
        | OpCode::NOP_10 => read_1!(Instruction::NOP),

        OpCode::RST_0
        | OpCode::RST_1
        | OpCode::RST_2
        | OpCode::RST_3
        | OpCode::RST_4
        | OpCode::RST_5
        | OpCode::RST_6
        | OpCode::RST_7 => read_1!(Instruction::RST),

        OpCode::EI => read_1!(Instruction::EI),
        OpCode::RET => read_1!(Instruction::RET),

        // LXI
        OpCode::LXI_B => read_3!(Instruction::LXI, buf, pos, B, D),
        OpCode::LXI_D => read_3!(Instruction::LXI, buf, pos, D, D),
        OpCode::LXI_H => read_3!(Instruction::LXI, buf, pos, H, D),
        OpCode::LXI_SP => read_3!(Instruction::LXI, buf, pos, SP, D),

        OpCode::JNC => read_3!(Instruction::JNC, buf, pos),

        OpCode::JMP => read_3!(Instruction::JMP, buf, pos),

        OpCode::STAX_B => read_3!(Instruction::STAX, buf, pos, B),
        OpCode::STAX_D => read_3!(Instruction::STAX, buf, pos, D),
        OpCode::STA => read_3!(Instruction::STA, buf, pos),

        OpCode::PUSH_PSW => read_1!(Instruction::PUSH(PSW)),
        OpCode::PUSH_B => read_1!(Instruction::PUSH(B)),
        OpCode::PUSH_D => read_1!(Instruction::PUSH(D)),
        OpCode::PUSH_H => read_1!(Instruction::PUSH(H)),

        // LDAX
        OpCode::LDAX_B => read_1!(Instruction::LDAX(B)),
        OpCode::LDAX_D => read_1!(Instruction::LDAX(D)),

        // Mvi
        OpCode::MVI_A => read_2!(Instruction::MVI, buf, pos, A, D),
        OpCode::MVI_B => read_2!(Instruction::MVI, buf, pos, B, D),
        OpCode::MVI_C => read_2!(Instruction::MVI, buf, pos, C, D),
        OpCode::MVI_D => read_2!(Instruction::MVI, buf, pos, D, D),
        OpCode::MVI_E => read_2!(Instruction::MVI, buf, pos, E, D),
        OpCode::MVI_H => read_2!(Instruction::MVI, buf, pos, H, D),
        OpCode::MVI_L => read_2!(Instruction::MVI, buf, pos, L, D),
        OpCode::MVI_M => read_2!(Instruction::MVI, buf, pos, M, D),

        // DCR
        OpCode::DCR_A => read_1!(Instruction::DCR(A)),
        OpCode::DCR_B => read_1!(Instruction::DCR(B)),
        OpCode::DCR_C => read_1!(Instruction::DCR(C)),
        OpCode::DCR_D => read_1!(Instruction::DCR(D)),
        OpCode::DCR_E => read_1!(Instruction::DCR(E)),
        OpCode::DCR_H => read_1!(Instruction::DCR(H)),
        OpCode::DCR_L => read_1!(Instruction::DCR(L)),
        OpCode::DCR_M => read_1!(Instruction::DCR(M)),

        // ORA
        OpCode::ORA_A => read_1!(Instruction::ORA(A)),
        OpCode::ORA_B => read_1!(Instruction::ORA(B)),
        OpCode::ORA_C => read_1!(Instruction::ORA(C)),
        OpCode::ORA_D => read_1!(Instruction::ORA(D)),
        OpCode::ORA_E => read_1!(Instruction::ORA(E)),
        OpCode::ORA_H => read_1!(Instruction::ORA(H)),
        OpCode::ORA_L => read_1!(Instruction::ORA(L)),
        OpCode::ORA_M => read_1!(Instruction::ORA(M)),

        OpCode::RRC => read_1!(Instruction::RRC),
        OpCode::RC => read_1!(Instruction::RC),
        OpCode::SUI => read_2!(Instruction::SUI, buf, pos),
        OpCode::ACI => read_2!(Instruction::ACI, buf, pos),

        OpCode::XTHL => read_1!(Instruction::XTHL),
        OpCode::PCHL => read_1!(Instruction::PCHL),

        OpCode::CALL => read_3!(Instruction::CALL, buf, pos),
        OpCode::CC => read_3!(Instruction::CC, buf, pos),
        OpCode::CPO => read_3!(Instruction::CPO, buf, pos),
        OpCode::CP => read_3!(Instruction::CP, buf, pos),

        OpCode::IN => read_2!(Instruction::IN, buf, pos),
        OpCode::ORI => read_2!(Instruction::ORI, buf, pos),
        OpCode::ADI => read_2!(Instruction::ADI, buf, pos),

        OpCode::JC => read_3!(Instruction::JC, buf, pos),
        OpCode::LDA => read_3!(Instruction::LDA, buf, pos),
        OpCode::JNZ => read_3!(Instruction::JNZ, buf, pos),

        // CMP
        OpCode::CMP_A => read_1!(Instruction::CMP(A)),
        OpCode::CMP_B => read_1!(Instruction::CMP(B)),
        OpCode::CMP_C => read_1!(Instruction::CMP(C)),
        OpCode::CMP_D => read_1!(Instruction::CMP(D)),
        OpCode::CMP_E => read_1!(Instruction::CMP(E)),
        OpCode::CMP_L => read_1!(Instruction::CMP(L)),
        OpCode::CMP_H => read_1!(Instruction::CMP(H)),
        OpCode::CMP_M => read_1!(Instruction::CMP(M)),

        // ANA
        OpCode::ANA_A => read_1!(Instruction::ANA(A)),
        OpCode::ANA_B => read_1!(Instruction::ANA(B)),
        OpCode::ANA_C => read_1!(Instruction::ANA(C)),
        OpCode::ANA_D => read_1!(Instruction::ANA(D)),
        OpCode::ANA_E => read_1!(Instruction::ANA(E)),
        OpCode::ANA_L => read_1!(Instruction::ANA(L)),
        OpCode::ANA_H => read_1!(Instruction::ANA(H)),
        OpCode::ANA_M => read_1!(Instruction::ANA(M)),

        OpCode::DAA => read_1!(Instruction::DAA),
        OpCode::STC => read_1!(Instruction::STC),
        OpCode::JZ => read_3!(Instruction::JZ, buf, pos),
        OpCode::CNZ => read_3!(Instruction::CNZ, buf, pos),
        OpCode::LHLD => read_3!(Instruction::LHLD, buf, pos),

        // XRA
        OpCode::XRA_A => read_1!(Instruction::XRA(A)),
        OpCode::XRA_B => read_1!(Instruction::XRA(B)),
        OpCode::XRA_C => read_1!(Instruction::XRA(C)),
        OpCode::XRA_D => read_1!(Instruction::XRA(D)),
        OpCode::XRA_E => read_1!(Instruction::XRA(E)),
        OpCode::XRA_L => read_1!(Instruction::XRA(L)),
        OpCode::XRA_H => read_1!(Instruction::XRA(H)),
        OpCode::XRA_M => read_1!(Instruction::XRA(M)),

        OpCode::CPI => read_2!(Instruction::CPI, buf, pos),
        OpCode::CNC => read_3!(Instruction::CNC, buf, pos),
        OpCode::OUT => read_2!(Instruction::OUT, buf, pos),

        OpCode::POP_H => read_1!(Instruction::POP(H)),
        OpCode::POP_B => read_1!(Instruction::POP(B)),
        OpCode::POP_D => read_1!(Instruction::POP(D)),
        OpCode::POP_PSW => read_1!(Instruction::POP(PSW)),
        OpCode::RZ => read_1!(Instruction::RZ),
        OpCode::RNC => read_1!(Instruction::RNC),
        OpCode::RNZ => read_1!(Instruction::RNZ),

        // Mov
        OpCode::MOV_A_A => read_1!(Instruction::MOV(A, A)),
        OpCode::MOV_A_B => read_1!(Instruction::MOV(A, B)),
        OpCode::MOV_A_C => read_1!(Instruction::MOV(A, C)),
        OpCode::MOV_A_D => read_1!(Instruction::MOV(A, D)),
        OpCode::MOV_A_E => read_1!(Instruction::MOV(A, E)),
        OpCode::MOV_A_H => read_1!(Instruction::MOV(A, H)),
        OpCode::MOV_A_L => read_1!(Instruction::MOV(A, L)),
        OpCode::MOV_A_M => read_1!(Instruction::MOV(A, M)),

        OpCode::MOV_B_A => read_1!(Instruction::MOV(B, A)),
        OpCode::MOV_B_B => read_1!(Instruction::MOV(B, B)),
        OpCode::MOV_B_C => read_1!(Instruction::MOV(B, C)),
        OpCode::MOV_B_D => read_1!(Instruction::MOV(B, D)),
        OpCode::MOV_B_E => read_1!(Instruction::MOV(B, E)),
        OpCode::MOV_B_H => read_1!(Instruction::MOV(B, H)),
        OpCode::MOV_B_L => read_1!(Instruction::MOV(B, L)),
        OpCode::MOV_B_M => read_1!(Instruction::MOV(B, M)),

        OpCode::MOV_C_A => read_1!(Instruction::MOV(C, A)),
        OpCode::MOV_C_B => read_1!(Instruction::MOV(C, B)),
        OpCode::MOV_C_C => read_1!(Instruction::MOV(C, C)),
        OpCode::MOV_C_D => read_1!(Instruction::MOV(C, D)),
        OpCode::MOV_C_E => read_1!(Instruction::MOV(C, E)),
        OpCode::MOV_C_H => read_1!(Instruction::MOV(C, H)),
        OpCode::MOV_C_L => read_1!(Instruction::MOV(C, L)),
        OpCode::MOV_C_M => read_1!(Instruction::MOV(C, M)),

        OpCode::MOV_D_A => read_1!(Instruction::MOV(D, A)),
        OpCode::MOV_D_B => read_1!(Instruction::MOV(D, B)),
        OpCode::MOV_D_C => read_1!(Instruction::MOV(D, C)),
        OpCode::MOV_D_D => read_1!(Instruction::MOV(D, D)),
        OpCode::MOV_D_E => read_1!(Instruction::MOV(D, E)),
        OpCode::MOV_D_H => read_1!(Instruction::MOV(D, H)),
        OpCode::MOV_D_L => read_1!(Instruction::MOV(D, L)),
        OpCode::MOV_D_M => read_1!(Instruction::MOV(D, M)),

        OpCode::MOV_E_A => read_1!(Instruction::MOV(E, A)),
        OpCode::MOV_E_B => read_1!(Instruction::MOV(E, B)),
        OpCode::MOV_E_C => read_1!(Instruction::MOV(E, C)),
        OpCode::MOV_E_D => read_1!(Instruction::MOV(E, D)),
        OpCode::MOV_E_E => read_1!(Instruction::MOV(E, E)),
        OpCode::MOV_E_H => read_1!(Instruction::MOV(E, H)),
        OpCode::MOV_E_L => read_1!(Instruction::MOV(E, L)),
        OpCode::MOV_E_M => read_1!(Instruction::MOV(E, M)),

        OpCode::MOV_H_A => read_1!(Instruction::MOV(H, A)),
        OpCode::MOV_H_B => read_1!(Instruction::MOV(H, B)),
        OpCode::MOV_H_C => read_1!(Instruction::MOV(H, C)),
        OpCode::MOV_H_D => read_1!(Instruction::MOV(H, D)),
        OpCode::MOV_H_E => read_1!(Instruction::MOV(H, E)),
        OpCode::MOV_H_H => read_1!(Instruction::MOV(H, H)),
        OpCode::MOV_H_L => read_1!(Instruction::MOV(H, L)),
        OpCode::MOV_H_M => read_1!(Instruction::MOV(H, M)),

        OpCode::MOV_L_A => read_1!(Instruction::MOV(L, A)),
        OpCode::MOV_L_B => read_1!(Instruction::MOV(L, B)),
        OpCode::MOV_L_C => read_1!(Instruction::MOV(L, C)),
        OpCode::MOV_L_D => read_1!(Instruction::MOV(L, D)),
        OpCode::MOV_L_E => read_1!(Instruction::MOV(L, E)),
        OpCode::MOV_L_H => read_1!(Instruction::MOV(L, H)),
        OpCode::MOV_L_L => read_1!(Instruction::MOV(L, L)),
        OpCode::MOV_L_M => read_1!(Instruction::MOV(L, M)),

        OpCode::MOV_M_A => read_1!(Instruction::MOV(M, A)),
        OpCode::MOV_M_B => read_1!(Instruction::MOV(M, B)),
        OpCode::MOV_M_C => read_1!(Instruction::MOV(M, C)),
        OpCode::MOV_M_D => read_1!(Instruction::MOV(M, D)),
        OpCode::MOV_M_E => read_1!(Instruction::MOV(M, E)),
        OpCode::MOV_M_H => read_1!(Instruction::MOV(M, H)),
        OpCode::MOV_M_L => read_1!(Instruction::MOV(M, L)),

        // INX
        OpCode::INX_B => read_1!(Instruction::INX(B)),
        OpCode::INX_D => read_1!(Instruction::INX(D)),
        OpCode::INX_SP => read_1!(Instruction::INX(SP)),
        OpCode::INX_H => read_1!(Instruction::INX(H)),

        OpCode::SHLD => read_3!(Instruction::SHLD, buf, pos),

        // DCX
        OpCode::DCX_B => read_1!(Instruction::DCX(B)),
        OpCode::DCX_D => read_1!(Instruction::DCX(D)),
        OpCode::DCX_H => read_1!(Instruction::DCX(H)),
        OpCode::DCX_SP => read_1!(Instruction::DCX(SP)),

        OpCode::INR_A => read_1!(Instruction::INR(A)),
        OpCode::INR_B => read_1!(Instruction::INR(B)),
        OpCode::INR_C => read_1!(Instruction::INR(C)),
        OpCode::INR_D => read_1!(Instruction::INR(D)),
        OpCode::INR_E => read_1!(Instruction::INR(E)),
        OpCode::INR_H => read_1!(Instruction::INR(H)),
        OpCode::INR_L => read_1!(Instruction::INR(L)),
        OpCode::INR_M => read_1!(Instruction::INR(M)),

        OpCode::ANI => read_2!(Instruction::ANI, buf, pos),
        OpCode::JPO => read_3!(Instruction::JPO, buf, pos),
        OpCode::CM => read_3!(Instruction::CM, buf, pos),
        OpCode::CPE => read_3!(Instruction::CPE, buf, pos),

        OpCode::RLC => read_1!(Instruction::RLC),
        OpCode::CZ => read_1!(Instruction::CZ),
        OpCode::RP => read_1!(Instruction::RP),
        OpCode::RPO => read_1!(Instruction::RPO),
        OpCode::RPE => read_1!(Instruction::RPE),

        OpCode::DAD_B => read_1!(Instruction::DAD(B)),
        OpCode::DAD_D => read_1!(Instruction::DAD(D)),
        OpCode::DAD_H => read_1!(Instruction::DAD(H)),
        OpCode::DAD_SP => read_1!(Instruction::DAD(SP)),

        OpCode::XCHG => read_1!(Instruction::XCHG),

        // ADD
        OpCode::ADD_A => read_1!(Instruction::ADD(A)),
        OpCode::ADD_B => read_1!(Instruction::ADD(B)),
        OpCode::ADD_C => read_1!(Instruction::ADD(C)),
        OpCode::ADD_D => read_1!(Instruction::ADD(D)),
        OpCode::ADD_E => read_1!(Instruction::ADD(E)),
        OpCode::ADD_H => read_1!(Instruction::ADD(H)),
        OpCode::ADD_L => read_1!(Instruction::ADD(L)),
        OpCode::ADD_M => read_1!(Instruction::ADD(M)),

        // ADC
        OpCode::ADC_A => read_1!(Instruction::ADC(A)),
        OpCode::ADC_B => read_1!(Instruction::ADC(B)),
        OpCode::ADC_C => read_1!(Instruction::ADC(C)),
        OpCode::ADC_D => read_1!(Instruction::ADC(D)),
        OpCode::ADC_E => read_1!(Instruction::ADC(E)),
        OpCode::ADC_H => read_1!(Instruction::ADC(H)),
        OpCode::ADC_L => read_1!(Instruction::ADC(L)),
        OpCode::ADC_M => read_1!(Instruction::ADC(M)),

        // SUB
        OpCode::SUB_A => read_1!(Instruction::SUB(A)),
        OpCode::SUB_B => read_1!(Instruction::SUB(B)),
        OpCode::SUB_C => read_1!(Instruction::SUB(C)),
        OpCode::SUB_D => read_1!(Instruction::SUB(D)),
        OpCode::SUB_E => read_1!(Instruction::SUB(E)),
        OpCode::SUB_H => read_1!(Instruction::SUB(H)),
        OpCode::SUB_L => read_1!(Instruction::SUB(L)),
        OpCode::SUB_M => read_1!(Instruction::SUB(M)),

        // SUB
        OpCode::SBB_A => read_1!(Instruction::SBB(A)),
        OpCode::SBB_B => read_1!(Instruction::SBB(B)),
        OpCode::SBB_C => read_1!(Instruction::SBB(C)),
        OpCode::SBB_D => read_1!(Instruction::SBB(D)),
        OpCode::SBB_E => read_1!(Instruction::SBB(E)),
        OpCode::SBB_H => read_1!(Instruction::SBB(H)),
        OpCode::SBB_L => read_1!(Instruction::SBB(L)),
        OpCode::SBB_M => read_1!(Instruction::SBB(M)),

        OpCode::CMA => read_1!(Instruction::CMA),
        OpCode::RIM => read_1!(Instruction::RIM),
        OpCode::SIM => read_1!(Instruction::SIM),
        OpCode::CMC => read_1!(Instruction::CMC),
        OpCode::RAL => read_1!(Instruction::RAL),
        OpCode::RAR => read_1!(Instruction::RAR),
        OpCode::RM => read_1!(Instruction::RM),

        OpCode::JM => read_3!(Instruction::JM, buf, pos),
        OpCode::JPE => read_3!(Instruction::JPE, buf, pos),
        OpCode::JP => read_3!(Instruction::JP, buf, pos),

        OpCode::SBI => read_2!(Instruction::SBI, buf, pos),
        OpCode::XRI => read_2!(Instruction::XRI, buf, pos),

        OpCode::SPHL => read_1!(Instruction::SPHL),
        e => Err(format!("OpCode unimplemented: {:?}", e)),
    }
}

pub fn reader(buf: Vec<u8>) -> OpReader {
    OpReader {
        buf: Memory::new(buf),
        pc: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_invaders() -> Vec<u8> {
        read_rom("roms/invaders.rom").unwrap()
    }

    #[test]
    fn test_read_rom() {
        assert_eq!(read_rom("roms/invaders.rom").unwrap().pop().unwrap(), 0x00);
    }

    #[test]
    fn test_diag() {
        let mut buf = read_rom("roms/cpudiag.bin").unwrap();
        let mut memory = vec![0x0; 256];

        memory[0] = 0xc3;
        memory[1] = 0;
        memory[2] = 0x01;
        memory.append(&mut buf);

        let mut r = reader(memory);

        let p = Path::new("disassemble_diag.txt");

        let result = r
            .read_all()
            .into_iter()
            .map(|(pos, ins)| format!("{:#X?} {:?}\n", pos, ins));
        fs::write(p, result.collect::<String>()).unwrap();
    }

    #[test]
    fn test_disassemble_all() {
        let buf = read_invaders();
        let mut r = reader(buf);

        let p = Path::new("roms/disassemble.txt");

        let result = r
            .read_all()
            .into_iter()
            .map(|(pos, ins)| format!("{:#X?} {:?}\n", pos, ins));
        fs::write(p, result.collect::<String>()).unwrap();
    }
}
