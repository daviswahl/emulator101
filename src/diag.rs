use crate::machine::display;
use crate::machine::memory;
use crate::machine::memory::Memory;
use crate::machine::rom::Rom;
use crate::machine::CPUInterface;
use crate::machine::MachineEvent;
use crate::machine::{Error, MachineInterface};
use crossbeam_channel::Sender;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;
use std::time::Instant;

#[allow(dead_code)]
struct Diag;

#[derive(Clone)]
pub struct DiagInterface {
    memory: Arc<RwLock<Memory>>,

    sender: Sender<[u8; display::FB_SIZE]>,
}

impl MachineInterface for DiagInterface {
    fn handle_in(&self, _cpu: &'_ mut CPUInterface<'_>, _port: u8) -> Result<(), Error> {
        Ok(())
    }

    fn handle_out(&self, _cpu: &'_ mut CPUInterface<'_>, _port: u8) -> Result<(), Error> {
        Ok(())
    }

    fn handle_interrupt(
        &self,
        _now: &'_ Instant,
        _cpu: &'_ mut CPUInterface<'_>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn memory_handle(&self) -> Result<RwLockWriteGuard<'_, Memory>, Error> {
        self.memory
            .write()
            .map_err(|_| Error::MemoryError(memory::Error::LockErr))
    }

    fn handle_event(&self, _evt: MachineEvent) -> Result<(), Error> {
        unimplemented!()
    }

    fn display_refresh(&self, _buf: [u8; display::FB_SIZE]) {}
    fn apply(memory: Arc<RwLock<Memory>>, sender: Sender<[u8; display::FB_SIZE]>) -> Self
    where
        Self: Sized,
    {
        DiagInterface { memory, sender }
    }
}

impl Rom<DiagInterface> for Diag {
    const DEBUG: bool = true;
    fn load<P: AsRef<Path>>(p: P) -> Result<Vec<u8>, String> {
        use std::io::Read;
        let mut fd = fs::File::open(p).map_err(|_| "bad rom path")?;
        let mut memory: [u8; 1709] = [0x0; 1453 + 256];
        fd.read(&mut memory).map_err(|_| "io error")?;
        memory.rotate_right(256);
        memory[0] = 0xc3;
        memory[1] = 0;
        memory[2] = 0x01;

        memory[368] = 0x7;

        memory[0x59D] = 0xc3;
        memory[0x59E] = 0xc2;
        memory[0x59F] = 0x5;
        Ok(memory.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::diag;

    #[test]
    fn test_diag() {
        match crate::machine::Machine::load::<diag::Diag>("roms/cpudiag.bin")
            .expect("couldn't load rom")
            .run()
        {
            Ok(_) => (),
            Err(err) => println!("{:?}", err),
        }
    }
}
