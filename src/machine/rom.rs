use crate::machine::MachineInterface;
use std::path::Path;

pub trait Rom<I: MachineInterface> {
    fn load<P: AsRef<Path>>(p: P) -> Result<Vec<u8>, String>;
}
