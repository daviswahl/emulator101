use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct Memory(Rc<RefCell<Vec<u8>>>);

impl Memory {
    pub fn new(vec: Vec<u8>) -> Self {
        Memory(Rc::new(RefCell::new(vec)))
    }

    pub fn read(&self, offset: u16) -> Result<u8, String> {
        let offset = offset as usize;
        let mem = self.0.borrow();
        if mem.len() > offset {
            Ok(mem[offset])
        } else {
            Err(format!(
                "Tried to read out of range address: {}, len: {}",
                offset,
                mem.len()
            ))
        }
    }

    pub fn len(&self) -> u16 {
        self.0.borrow().len() as u16
    }

    pub fn write(&mut self, offset: u16, data: u8) -> Result<(), String> {
        let offset = offset as usize;
        let mut mem = self.0.borrow_mut();
        if mem.len() > offset {
            mem[offset] = data;
            Ok(())
        } else {
            Err(format!(
                "Tried to set out of range address: {}, len: {}",
                offset,
                mem.len()
            ))
        }
    }
}
