use machine::CPUInterface;

pub struct SpaceInavdersInterruptHandler;
use machine::CPU;

impl CPUInterface for SpaceInavdersInterruptHandler {
    fn handle_in(cpu: &mut CPU, data: u8) -> Result<(), String> {
        Ok(())
    }

    fn handle_out(cpu: &mut CPU, data: u8) -> Result<(), String> {
        Ok(())
    }

    fn apply() -> Self {
        SpaceInavdersInterruptHandler
    }
}
