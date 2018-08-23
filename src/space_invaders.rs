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
use std::fs;
use std::path::Path;
use std::sync;

fn lock_err<T>(_err: sync::PoisonError<T>) -> String {
    "could not obtain lock".to_string()
}
impl MachineInterface for SpaceInvadersMachineInterface {
    fn handle_in(&self, cpu: &mut CPUInterface, port: u8) -> Result<(), String> {
        cpu.cpu.a = match port {
            0 => 1,
            1 => 0,
            3 => {
                let read = self.state.read().map_err(lock_err)?;
                let v = u16::from(read.shift1) << 8 | u16::from(read.shift0);

                ((v >> 8u8.wrapping_sub(read.shift_offset)) & 0xff) as u8
            }
            _ => 0,
        };

        Ok(())
    }

    fn handle_out(&self, cpu: &mut CPUInterface, port: u8) -> Result<(), String> {
        let value = cpu.cpu.a;
        let mut state = self.state.write().map_err(lock_err)?;
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

    fn handle_interrupt(&self, now: &Instant, cpu: &mut CPUInterface) -> Result<(), String> {
            if cpu.cpu.int_enable == 1 && self.state.read().map_err(lock_err)?.next_interrupt <= *now {
                let mut write = self.state.write().map_err(lock_err)?;
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

    fn memory_handle(&self) -> Result<RwLockWriteGuard<Memory>, String> {
        self.memory
            .write()
            .map_err(|_| "Failed to obtain memory lock".to_owned())
    }

    fn display_refresh(&self, buf: [u8; display::FB_SIZE]) -> Result<(), String> {
        self.sender.send(buf);
        Ok(())
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
    fn load<P: AsRef<Path>>(p: P) -> Result<Vec<u8>, String> {
        fs::read(p).map_err(|_| "failed to read space invaders rom".to_owned())
    }
}
