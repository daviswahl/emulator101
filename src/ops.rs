#[repr(u8)]
#[derive(FromPrimitive, PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum OpCode {
    NOP = 0x00,
    LXI_B_D = 0x01,
    STAX = 0x02,
    JMP = 0xc3,
    PUSH_PSW = 0xf5,
    PUSH_B = 0xc5,
    PUSH_D = 0xd5,
    PUSH_H = 0xe5,
    MVI = 0x3e,
    STA = 0x32,
    LXI_H_D = 0x21,
}

#[derive(PartialEq, Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    PSW,
}

#[derive(PartialEq, Debug)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    NOP,
    LXI([Register; 2], u8, u8),
    JMP(u8, u8),
    PUSH(Register),
    MVI(u8),
    STA(u8, u8),
}
