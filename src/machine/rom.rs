use crate::machine::cpu::disassembler::disassemble;
use crate::machine::memory::Memory;
use crate::machine::MachineInterface;
use std::path::Path;

pub trait Rom<I: MachineInterface> {
    const DEBUG: bool;
    fn load<P: AsRef<Path>>(p: P) -> Result<Vec<u8>, String>;
    fn dissassembble<P: AsRef<Path>>(p: P) -> Result<String, String> {
        let buf = Self::load(p)?;
        let mem = Memory::new(buf);

        let mut pc = 0;

        let mut s = String::new();
        while pc < mem.len() {
            let (inst, inc) = disassemble(&mem, pc).unwrap();
            s.push_str(format!("{:#X?} {}\n", pc, inst).as_str());
            pc += inc;
        }
        Ok(s)
    }
}
