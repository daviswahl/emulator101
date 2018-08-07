use ops::*;

use num::FromPrimitive;
use std::fs;
use std::path::Path;

pub fn read_rom(p: &'static str) -> Result<Vec<u8>, &'static str> {
    fs::read(Path::new(p)).map_err(|_| "failed to read file")
}

pub struct OpReader<I>(I);

fn read_code<I: Iterator<Item = u8>>(code: OpCode, iter: &mut I) -> Result<Instruction, String> {
    match code {
        OpCode::NOP => Ok(Instruction::NOP),
        OpCode::LXI => {
            let next = iter.next().unwrap();
            let next2 = iter.next().unwrap();
            Ok(Instruction::LXI(next, next2))
        }

        OpCode::JMP => {
            let n = iter.next().unwrap();
            let n2 = iter.next().unwrap();
            Ok(Instruction::JMP(n, n2))
        }

        e => Err(format!("OpCode unimplemetned: {:?}", e)),
    }
}

impl<I: Iterator<Item = u8>> Iterator for OpReader<I> {
    type Item = Result<Instruction, String>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(b) = self.0.next() {
            if let Some(code) = OpCode::from_u8(b) {
                Some(read_code(code, &mut self.0))
            } else {
                Some(Err(format!("OpCode unimplemetned: {:?}", b)))
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
}
