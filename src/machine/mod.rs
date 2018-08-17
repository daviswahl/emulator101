mod cpu;
mod memory;

use machine::memory::Memory;
use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::rc::Rc;

pub struct Machine {
    cpu: cpu::CPU,
    memory: memory::Memory,
}

impl Machine {
    pub fn with_rom(path: &'static str) -> Result<Machine, &'static str> {
        let mut v = fs::read(Path::new(path)).map_err(|_| "failed to read file")?;
        v.extend(vec![0x0; 8192]);
        let memory = Memory::new(v);
        let cpu = cpu::new_state(memory.clone());

        Ok(Machine { memory, cpu })
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.cpu.process(|interupt| {
            println!("got interrupt: {:?}", interupt);
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::ops::IndexMut;
    use std::rc::Rc;

    #[derive(Clone)]
    struct Memory(Rc<RefCell<Vec<u8>>>);
    impl Memory {
        fn write(&self, offset: u16, data: u8) -> Result<(), String> {
            self.0.borrow_mut()[offset as usize] = data;
            Ok(())
        }

        fn read(&self) -> u8 {
            self.0.borrow()[0]
        }
    }

    struct CPU(Memory);

    impl CPU {
        fn run(&mut self) {
            self.0.write(0x0, 0x1);
        }
    }
    struct Machine {
        cpu: CPU,
        memory: Memory,
    }

    impl Machine {
        fn run(&mut self) {
            self.cpu.run()
        }

        fn read(&self) -> u8 {
            self.memory.read()
        }
    }

    #[test]
    fn test() {
        let memory = Memory(Rc::new(RefCell::new(vec![0x0])));
        let mut machine = Machine {
            cpu: CPU(memory.clone()),
            memory: memory,
        };

        machine.run();
        assert_eq!(machine.read(), 0x1)
    }

}
