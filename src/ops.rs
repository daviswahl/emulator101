#[repr(u8)]
#[derive(FromPrimitive, PartialEq, Debug)]
pub enum OpCode {
    NOP = 0x00,
    LXI = 0x01,
    STAX = 0x02,
    JMP = 0xc3,
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    NOP,
    LXI(u8, u8),
    JMP(u8, u8),
}
