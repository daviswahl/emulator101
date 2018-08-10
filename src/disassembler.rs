use ops::*;

use num::FromPrimitive;
use std::fs;
use std::path::Path;

pub fn read_rom(p: &'static str) -> Result<Vec<u8>, &'static str> {
    fs::read(Path::new(p)).map_err(|_| "failed to read file")
}

pub struct OpReader<I>(I);

macro_rules! read_2 {
    ($inst:path, $iter:ident) => {
        Ok($inst($iter.next().unwrap()))
    };
}

macro_rules! read_3 {
    ($inst:path, $iter:ident) => {
        Ok($inst($iter.next().unwrap(), $iter.next().unwrap()))
    };
    ($inst:path, $iter:ident,$reg:expr) => {
        Ok($inst($reg, $iter.next().unwrap(), $iter.next().unwrap()))
    };
}

fn read_code<I>(code: OpCode, iter: &mut I) -> Result<Instruction, String>
where
    I: Iterator<Item = u8>,
{
    use ops::Register::*;
    match code {
        OpCode::NOP => Ok(Instruction::NOP),
        OpCode::LXI_B_D => read_3!(Instruction::LXI, iter, [B, D]),
        OpCode::LXI_H_D => read_3!(Instruction::LXI, iter, [H, D]),

        OpCode::JMP => read_3!(Instruction::JMP, iter),
        OpCode::MVI => read_2!(Instruction::MVI, iter),
        OpCode::STA => read_3!(Instruction::STA, iter),

        OpCode::PUSH_PSW => Ok(Instruction::PUSH(PSW)),
        OpCode::PUSH_B => Ok(Instruction::PUSH(B)),
        OpCode::PUSH_D => Ok(Instruction::PUSH(D)),
        OpCode::PUSH_H => Ok(Instruction::PUSH(H)),

        e => Err(format!("OpCode unimplemented: {:?}", e)),
    }
}

impl<I> Iterator for OpReader<I>
where
    I: Iterator<Item = u8>,
{
    type Item = Result<Instruction, String>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b) = self.0.next() {
            if let Some(code) = OpCode::from_u8(b) {
                Some(read_code(code, &mut self.0))
            } else {
                Some(Err(format!("OpCode unimplemented: {:X?}", b)))
            }
        } else {
            None
        }
    }
}

pub fn reader(buf: Vec<u8>) -> OpReader<impl Iterator<Item = u8>> {
    OpReader(buf.into_iter())
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
        assert_eq!(r.next(), Some(Ok(Instruction::NOP)));
        r.next();
        r.next();
        assert_eq!(r.next(), Some(Ok(Instruction::JMP(0xd4, 0x18))));
    }

    #[test]
    fn test_disassemble_all() {
        let buf = read_invaders();
        let r = reader(buf);

        r.for_each(|inst| println!("Instruction: {:?}", inst.unwrap()));
    }
}
