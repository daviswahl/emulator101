use crate::machine::display;

#[derive(Debug)]
pub struct Memory(Vec<u8>);

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Out of range access: {}, len: {}", _0, _1)]
    OutOfRangeAccess(usize, usize),
    #[fail(display = "LockError")]
    LockErr,
}

impl Memory {
    pub fn new(vec: Vec<u8>) -> Self {
        Memory(vec)
    }

    pub fn read(&self, offset: u16) -> Result<u8, Error> {
        let offset = offset as usize;
        let mem = &self.0;
        if mem.len() > offset {
            Ok(mem[offset])
        } else {
            Err(Error::OutOfRangeAccess(offset, mem.len()))
        }
    }

    pub fn len(&self) -> u16 {
        self.0.len() as u16
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn vram(&self) -> Result<[u8; display::FB_SIZE], Error> {
        let mut v = [0; display::FB_SIZE];
        if self.0.len() >= 0x4000 {
            v.copy_from_slice(&self.0[0x2400..0x4000]);
            Ok(v)
        } else {
            Err(Error::OutOfRangeAccess(0x4000, self.0.len()))
        }
    }

    pub fn write(&mut self, offset: u16, data: u8) -> Result<(), Error> {
        let offset = offset as usize;
        let mem = &mut self.0;
        if mem.len() > offset {
            mem[offset] = data;
            Ok(())
        } else {
            Err(Error::OutOfRangeAccess(offset, mem.len()))
        }
    }
}
