use ops::*;

use num::FromPrimitive;
use std::fs;
use std::iter::Enumerate;
use std::path::Path;

pub fn read_rom(p: &'static str) -> Result<Vec<u8>, &'static str> {
    fs::read(Path::new(p)).map_err(|_| "failed to read file")
}

pub struct OpReader<I> {
    iter: I,
    pc: i8,
}

macro_rules! read_1 {
    ($inst:expr) => {
        Ok(($inst, 0))
    };
}
macro_rules! read_2 {
    ($inst:path, $iter:ident) => {
        Ok(($inst($iter.next().unwrap()), 1))
    };
}

macro_rules! read_3 {
    ($inst:path, $iter:ident) => {
        Ok(($inst($iter.next().unwrap(), $iter.next().unwrap()), 2))
    };
    ($inst:path, $iter:ident,$reg:expr) => {
        Ok(($inst($reg, $iter.next().unwrap(), $iter.next().unwrap()), 2))
    };
}

fn read_code<I>(code: OpCode, iter: &mut I) -> Result<(Instruction, i8), String>
where
    I: Iterator<Item = u8>,
{
    use ops::Register::*;
    match code {
        OpCode::NOP => read_1!(Instruction::NOP),
        OpCode::LXI_B_D => read_3!(Instruction::LXI, iter, [B, D]),
        OpCode::LXI_H_D => read_3!(Instruction::LXI, iter, [H, D]),

        OpCode::JMP => read_3!(Instruction::JMP, iter),
        OpCode::MVI => read_2!(Instruction::MVI, iter),
        OpCode::STA => read_3!(Instruction::STA, iter),

        OpCode::PUSH_PSW => read_1!(Instruction::PUSH(PSW)),
        OpCode::PUSH_B => read_1!(Instruction::PUSH(B)),
        OpCode::PUSH_D => read_1!(Instruction::PUSH(D)),
        OpCode::PUSH_H => read_1!(Instruction::PUSH(H)),

        e => Err(format!("OpCode unimplemented: {:?}", e)),
    }
}

impl<I> Iterator for OpReader<I>
where
    I: Iterator<Item = u8>,
{
    type Item = Result<(Instruction, i8), String>;
    fn next(&mut self) -> Option<Self::Item> {
        let current_pc = self.pc;
        self.pc += 1;

        if let Some(b) = self.iter.next() {
            if let Some(code) = OpCode::from_u8(b) {
                if let Ok((inst, i)) = read_code(code, &mut self.iter) {
                    self.pc += i;
                    Some(Ok((inst, current_pc)))
                } else {
                    None
                }
            } else {
                Some(Err(format!("OpCode unimplemented: {:#X?}", b)))
            }
        } else {
            None
        }
    }
}

impl<I> OpReader<I> where I: Iterator<Item = u8> {}

pub fn reader(buf: Vec<u8>) -> OpReader<impl Iterator<Item = u8>> {
    OpReader {
        iter: buf.into_iter(),
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
    fn test_disassemble() {
        let buf = read_invaders();
        let mut r = reader(buf);
        assert_eq!(r.next(), Some(Ok((Instruction::NOP, 1))));
        r.next();
        r.next();
        assert_eq!(r.next(), Some(Ok((Instruction::JMP(0xd4, 0x18), 2))));
    }

    #[test]
    fn test_disassemble_all() {
        let buf = read_invaders();
        let r = reader(buf);

        r.for_each(|inst| {
            let (inst, pc) = inst.unwrap();
            println!("{:#X?} {:?}", pc, inst)
        })
    }
}
