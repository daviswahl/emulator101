pub mod cpu;
pub mod display;
mod error;
pub mod memory;
pub mod rom;

pub use error::Error;

pub use crate::machine::cpu::pause;
pub use crate::machine::cpu::CPUInterface;
pub use crate::machine::cpu::CPU;
use crate::machine::memory::Memory;
use crate::machine::rom::Rom;
use crossbeam_channel as channel;
use crossbeam_channel::Sender;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockWriteGuard;
use std::thread;
use std::time;
use std::time::Duration;

pub trait MachineInterface: Clone {
    fn handle_in(&self, cpu: &mut CPUInterface, port: u8) -> Result<(), Error>;
    fn handle_out(&self, cpu: &mut CPUInterface, port: u8) -> Result<(), Error>;
    fn handle_interrupt(&self, now: &time::Instant, cpu: &mut CPUInterface) -> Result<(), Error>;
    fn memory_handle(&self) -> Result<RwLockWriteGuard<Memory>, Error>;

    fn display_refresh(&self, buf: [u8; display::FB_SIZE]);
    fn apply(memory: Arc<RwLock<Memory>>, sender: Sender<[u8; display::FB_SIZE]>) -> Self
    where
        Self: Sized;
}

pub struct Machine<I> {
    cpu: Arc<RwLock<cpu::CPU>>,
    memory: Arc<RwLock<memory::Memory>>,
    interface: PhantomData<*const I>,
}

impl<I: MachineInterface + Send + 'static> Machine<I> where {
    pub fn load<R: Rom<I>>(path: &'static str) -> Result<Machine<I>, &'static str> {
        let rom = R::load(path).map_err(|_| "failed to read file")?;
        let mut buf = vec![0x0; 8 * 0x1000];
        buf.splice(..rom.len(), rom.into_iter());

        let memory = Arc::new(RwLock::new(Memory::new(buf)));
        let mut cpu = cpu::new();
        cpu.debug = R::DEBUG;
        let cpu = Arc::new(RwLock::new(cpu));

        Ok(Machine {
            memory,
            cpu,
            interface: PhantomData,
        })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let (tx, rx) = channel::unbounded();
        let memory = self.memory.clone();
        let cpu = self.cpu.clone();
        let interface = I::apply(memory, tx);
        let interface2 = interface.clone();
        let debug = self.cpu.read()?.debug;

        let th1: thread::JoinHandle<Result<(), Error>> = thread::spawn(move || {
            let start = time::Instant::now();
            let mut last_timer = start;

            let timer = channel::tick(Duration::from_millis(1));

            let mut iters = 0;
            while let Some(now) = timer.recv() {
                let mut cpu_interface = CPUInterface {
                    cpu: &mut *cpu.write()?,
                    memory: &mut *interface.memory_handle()?,
                };

                let since_last = now - last_timer;
                let cycles_behind = 2 * since_last.as_micros();

                let mut cycles = 0;
                while cycles < cycles_behind {
                    cycles += u128::from(crate::machine::cpu::emulate(
                        &mut cpu_interface,
                        &interface,
                    )?);
                }

                interface.handle_interrupt(&now, &mut cpu_interface)?;
                if iters % 100 == 0 {
                    let mhz = cpu_interface.cpu.cycles as f64 / start.elapsed().as_micros() as f64;
                    println!("mhz: {}", mhz);
                }

                if iters % 1000 == 0 {
                    //println!("{}", cpu_interface.cpu.history)
                }

                last_timer = now;
                iters += 1;
            }
            Ok(())
        });

        let th2 = thread::spawn(move || {
            let timer = channel::tick(Duration::from_millis(16));
            while let Some(_) = timer.recv() {
                interface2.display_refresh(interface2.memory_handle()?.vram()?)
            }
            Ok(())
        });

        if !debug {
            display::run(rx)?;
        }

        th1.join()??;
        th2.join()?
    }
}
