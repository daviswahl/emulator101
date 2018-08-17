mod cpu;
mod memory;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Machine {
    cpu: cpu::CPU,
}


#[cfg(test)]
mod test {

    use std::cell::Cell;
    use std::rc::Rc;
    use std::ops::IndexMut;
    use std::cell::RefCell;


    #[derive(Clone)]
    struct Memory(Rc<RefCell<Vec<u8>>>);
    impl Memory {
        fn write (&self, offset: u16, data: u8) -> Result<(), String> {
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
