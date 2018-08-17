pub(crate) struct Memory(pub(crate) Vec<u8>);

impl Memory {
    pub fn raw(&self) -> &[u8] {
        &self.0
    }
    pub fn read(&self, offset: u16) -> Result<u8, String> {
        let offset = offset as usize;
        if self.0.len() > offset {
            Ok(self.0[offset])
        } else {
            Err(format!(
                "Tried to read out of range address: {}, len: {}",
                offset,
                self.0.len()
            ))
        }
    }

    pub fn write(&mut self, offset: u16, data: u8) -> Result<(), String> {
        let offset = offset as usize;
        if self.0.len() > offset {
            self.0[offset] = data;
            Ok(())
        } else {
            Err(format!(
                "Tried to set out of range address: {}, len: {}",
                offset,
                self.0.len()
            ))
        }
    }
}
