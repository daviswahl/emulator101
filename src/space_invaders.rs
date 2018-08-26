use crate::machine::display;
use crate::machine::memory::Memory;
use crate::machine::CPUInterface;
use crate::machine::MachineInterface;
use crossbeam_channel::Sender;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;
use std::time::Duration;
use std::time::Instant;

pub struct SpaceInvadersMachineState {
    shift0: u8,
    shift1: u8,
    shift_offset: u8,
    next_interrupt: Instant,
    which_interrupt: u8,
}

#[derive(Clone)]
pub struct SpaceInvadersMachineInterface {
    state: Arc<RwLock<SpaceInvadersMachineState>>,

    memory: Arc<RwLock<Memory>>,

    sender: Sender<[u8; display::FB_SIZE]>,
}

pub struct SpaceInvaders;
use crate::machine::rom::Rom;
use crate::machine::Error;
use std::fs;
use std::path::Path;

impl MachineInterface for SpaceInvadersMachineInterface {
    fn handle_in(&self, cpu: &mut CPUInterface, port: u8) -> Result<(), Error> {
        cpu.cpu.a = match port {
            0 => 1,
            1 => 0,
            3 => {
                let read = self.state.read()?;
                let v = u16::from(read.shift1) << 8 | u16::from(read.shift0);

                ((v >> 8u8.wrapping_sub(read.shift_offset)) & 0xff) as u8
            }
            _ => 0,
        };

        Ok(())
    }

    fn handle_out(&self, cpu: &mut CPUInterface, port: u8) -> Result<(), Error> {
        let value = cpu.cpu.a;
        let mut state = self.state.write()?;
        match port {
            2 => state.shift_offset = value & 0x7,
            4 => {
                state.shift0 = state.shift1;
                state.shift1 = value;
            }
            _ => (),
        }
        Ok(())
    }

    fn handle_interrupt(&self, now: &Instant, cpu: &mut CPUInterface) -> Result<(), Error> {
        if cpu.cpu.int_enable == 1 && self.state.read()?.next_interrupt <= *now {
            let mut write = self.state.write()?;
            if write.which_interrupt == 1 {
                write.which_interrupt = 2;
                cpu.interrupt(1)?;
            } else {
                write.which_interrupt = 1;
                cpu.interrupt(2)?;
            }
            write.next_interrupt = *now + Duration::from_micros(8000);
        }
        Ok(())
    }
    fn memory_handle(&self) -> Result<RwLockWriteGuard<Memory>, Error> {
        Ok(self.memory.write()?)
    }

    fn display_refresh(&self, buf: [u8; display::FB_SIZE]) {
        self.sender.send(buf)
    }

    fn apply(memory: Arc<RwLock<Memory>>, sender: Sender<[u8; display::FB_SIZE]>) -> Self {
        let now = Instant::now();
        let state = Arc::new(RwLock::new(SpaceInvadersMachineState {
            shift0: 0,
            shift1: 0,
            shift_offset: 0,

            next_interrupt: now + Duration::from_micros(16000),
            which_interrupt: 1,
        }));
        SpaceInvadersMachineInterface {
            state,

            memory,
            sender,
        }
    }
}
impl Rom<SpaceInvadersMachineInterface> for SpaceInvaders {
    const DEBUG: bool = false;

    fn load<P: AsRef<Path>>(p: P) -> Result<Vec<u8>, String> {
        fs::read(p).map_err(|_| "failed to read space invaders rom".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dissassemble_all() {
        let buf = SpaceInvaders::dissassembble("roms/invaders.rom").unwrap();
        ::std::fs::write(std::path::Path::new("disassemble.txt"), buf).unwrap();
    }
}
