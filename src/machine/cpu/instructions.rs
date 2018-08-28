use crate::machine::cpu::ops::Register;
use crate::machine::cpu::ops::Register::*;
use crate::machine::cpu::CPUInterface;
use crate::machine::cpu::{Error, ErrorKind};

type OpResult = Result<u8, Error>;

/// ADD
pub(crate) fn add(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let mut cycles = 4;
    let answer = match reg {
        Register::M => {
            cycles = 7;
            let offset = to_adr(state.cpu.h, state.cpu.l);
            let m: u16 = state.read(offset)?.into();
            u16::from(state.cpu.a) + m
        }
        r => (u16::from(state.cpu.a)) + (u16::from(state.get_u8(r))),
    };
    state.cpu.cc.arith_flags(answer);
    state.cpu.a = (answer & 0xff) as u8;
    Ok(cycles)
}

pub(crate) fn aci(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let db: u16 = state.read_1()?.into();
    let carry = if state.cpu.cc.cy { 1 } else { 0 };
    let a: u16 = state.cpu.a.into();
    let result = a.wrapping_add(db.wrapping_add(carry));
    state.cpu.a = (result & 0xff) as u8;
    state.cpu.cc.flags_zsp((result & 0xff) as u8);
    state.cpu.cc.cy = result > 255;
    Ok(7)
}

pub(crate) fn adc(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let carry = if state.cpu.cc.cy { 1 } else { 0 };
    let mut cycles = 4;
    let answer = match reg {
        Register::M => {
            cycles = 7;
            let offset = to_adr(state.cpu.h, state.cpu.l);
            let m = u16::from(state.read(offset)?).wrapping_add(carry);
            u16::from(state.cpu.a).wrapping_add(m)
        }
        r => u16::from(state.cpu.a)
            .wrapping_add(u16::from(state.get_u8(r)))
            .wrapping_add(carry),
    };

    state.cpu.cc.arith_flags(answer);
    state.cpu.a = (answer & 0xff) as u8;
    Ok(cycles)
}

pub(crate) fn adi(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let val = state.read_1()?;
    let answer = (u16::from(state.cpu.a)) + u16::from(val);

    state.cpu.cc.flags_zsp((answer & 0xff) as u8);
    state.cpu.cc.cy = answer > 255;
    state.cpu.a = (answer & 0xff) as u8;
    Ok(7)
}

pub(crate) fn dad(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let answer = match reg {
        SP => {
            let hl: u32 = to_adr(state.cpu.h, state.cpu.l).into();
            hl.wrapping_add(state.cpu.sp.into())
        }
        B => {
            let bc: u32 = to_adr(state.cpu.b, state.cpu.c).into();
            let hl: u32 = to_adr(state.cpu.h, state.cpu.l).into();
            hl.wrapping_add(bc)
        }
        D => {
            let de: u32 = to_adr(state.cpu.d, state.cpu.e).into();
            let hl: u32 = to_adr(state.cpu.h, state.cpu.l).into();
            hl.wrapping_add(de)
        }
        H => {
            let hl: u32 = to_adr(state.cpu.h, state.cpu.l).into();
            hl.wrapping_add(hl)
        }
        s => unimplemented!("unimplemented lxi: {:?}", s),
    };

    state.cpu.cc.cy = (answer & 0xffff0000) > 0;

    state.cpu.h = ((answer & 0xff00) >> 8) as u8;
    state.cpu.l = (answer & 0xff) as u8;
    Ok(10)
}

/// SUBTRACT
pub(crate) fn sub(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let mut cycles = 4;
    let answer = match reg {
        Register::M => {
            cycles = 7;
            let offset = to_adr(state.cpu.h, state.cpu.l);
            let m = u16::from(state.read(offset)?);
            (u16::from(state.cpu.a).wrapping_sub(m))
        }
        r => u16::from(state.cpu.a).wrapping_sub(u16::from(state.get_u8(r))),
    };
    state.cpu.cc.arith_flags(answer);
    state.cpu.a = (answer & 0xff) as u8;
    Ok(cycles)
}

pub(crate) fn sui(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let db = state.read_1()?;
    let a = state.cpu.a;
    let result = a.wrapping_sub(db);
    state.cpu.a = result;
    state.cpu.cc.flags_zsp(result);
    state.cpu.cc.cy = a < db;
    Ok(7)
}

pub(crate) fn sbi(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let db: u16 = state.read_1()?.into();
    let carry = if state.cpu.cc.cy { 1 } else { 0 };
    let a: u16 = state.cpu.a.into();
    let result = a.wrapping_sub(db).wrapping_sub(carry);
    state.cpu.a = (result & 0xff) as u8;
    state.cpu.cc.flags_zsp((result & 0xff) as u8);
    state.cpu.cc.cy = result > 255;
    Ok(7)
}

pub(crate) fn sbb(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let mut cycles = 4;
    let carry = if state.cpu.cc.cy { 1 } else { 0 };
    let answer = match reg {
        Register::M => {
            cycles = 7;
            let offset = to_adr(state.cpu.h, state.cpu.l);
            let m: u16 = state.read(offset)?.into();
            u16::from(state.cpu.a).wrapping_sub(m).wrapping_sub(carry)
        }
        r => u16::from(state.cpu.a)
            .wrapping_sub(u16::from(state.get_u8(r)))
            .wrapping_sub(carry),
    };

    state.cpu.cc.arith_flags(answer);
    state.cpu.a = (answer & 0xff) as u8;
    Ok(cycles)
}

/// INCREMENT / DECREMENT
pub(crate) fn inr(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let mut cycles = 5;
    let answer = match reg {
        Register::M => {
            cycles = 10;
            let offset = to_adr(state.cpu.h, state.cpu.l);
            let result = state.read(offset)?.wrapping_add(1);
            write_hl(state, result)?;
            result
        }
        r => {
            let result = state.get_u8(r).wrapping_add(1);
            state.set_u8(reg, result);
            result
        }
    };
    state.cpu.cc.flags_zsp(answer);
    Ok(cycles)
}

pub(crate) fn ani(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let data: u16 = state.read_1()?.into();
    let answer = (u16::from(state.cpu.a)) & data;

    state.cpu.cc.logic_flags((answer & 0xff) as u8);
    state.cpu.a = (answer & 0xff) as u8;
    Ok(7)
}

pub(crate) fn inx(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    match &reg {
        B => {
            state.cpu.c = state.cpu.c.wrapping_add(1);
            if state.cpu.c == 0 {
                state.cpu.b = state.cpu.b.wrapping_add(1)
            }
        }
        D => {
            state.cpu.e = state.cpu.e.wrapping_add(1);
            if state.cpu.e == 0 {
                state.cpu.d = state.cpu.d.wrapping_add(1);
            }
        }
        SP => {
            state.cpu.sp = state.cpu.sp.wrapping_add(1);
        }
        H => {
            state.cpu.l = state.cpu.l.wrapping_add(1);
            if state.cpu.l == 0 {
                state.cpu.h = state.cpu.h.wrapping_add(1);
            }
        }
        _ => unimplemented!("unimplemented inx: {:?}", reg),
    };
    Ok(5)
}

pub(crate) fn dcx(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    match &reg {
        B => {
            state.cpu.c = state.cpu.c.wrapping_sub(1);
            if state.cpu.c == 0xff {
                state.cpu.b = state.cpu.b.wrapping_sub(1);
            }
        }
        D => {
            state.cpu.e = state.cpu.e.wrapping_sub(1);
            if state.cpu.e == 0xff {
                state.cpu.d = state.cpu.d.wrapping_sub(1);
            }
        }
        SP => {
            state.cpu.sp = state.cpu.sp.wrapping_sub(1);
        }
        H => {
            state.cpu.l = state.cpu.l.wrapping_sub(1);
            if state.cpu.l == 0xff {
                state.cpu.h = state.cpu.h.wrapping_sub(1);
            }
        }
        _ => unimplemented!("unimplemented inx: {:?}", reg),
    };
    Ok(5)
}

pub(crate) fn dcr(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let mut cycles = 5;
    let answer = match reg {
        Register::M => {
            cycles = 10;
            let offset = to_adr(state.cpu.h, state.cpu.l);
            let m: u16 = state.read(offset)?.into();
            let result = (m.wrapping_sub(1) & 0xff) as u8;
            write_hl(state, result)?;
            result
        }
        r => {
            let result = (u16::from(state.get_u8(r)).wrapping_sub(1) & 0xff) as u8;
            state.set_u8(reg, result);
            result
        }
    };
    state.cpu.cc.flags_zsp(answer);
    Ok(cycles)
}

pub(crate) fn lxi(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    match reg {
        SP => {
            let l = state.read_1()?;
            let h = state.read_1()?;
            state.cpu.sp = to_adr(h, l);
        }
        B => {
            state.cpu.c = state.read_1()?;
            state.cpu.b = state.read_1()?;
        }
        D => {
            state.cpu.e = state.read_1()?;
            state.cpu.d = state.read_1()?;
        }
        H => {
            state.cpu.l = state.read_1()?;
            state.cpu.h = state.read_1()?;
        }
        s => unimplemented!("unimplemented lxi: {:?}", s),
    };
    Ok(10)
}

pub(crate) fn lda(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let adr = read_2_address(state)?;
    state.cpu.a = state.read(adr)?;
    Ok(13)
}

pub(crate) fn sta(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let a = state.cpu.a;
    let adr = read_2_address(state)?;
    state.write(adr, a)?;
    Ok(13)
}

// 	L <- (adr); H<-(adr+1)
pub(crate) fn lhld(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let l = state.read_1()?;
    let h = state.read_1()?;
    let adr = to_adr(h, l);
    state.cpu.l = state.read(adr)?;
    state.cpu.h = state.read(adr + 1)?;
    Ok(16)
}

// 	(adr) <-L; (adr+1)<-H
pub(crate) fn shld(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let l = state.read_1()?;
    let h = state.read_1()?;
    let adr = to_adr(h, l);
    state.write(adr, state.cpu.l)?;
    state.write(adr.wrapping_add(1), state.cpu.h)?;
    Ok(16)
}

pub(crate) fn xchg(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let d = state.cpu.d;
    let e = state.cpu.e;
    state.cpu.d = state.cpu.h;
    state.cpu.e = state.cpu.l;
    state.cpu.h = d;
    state.cpu.l = e;
    Ok(4)
}

pub(crate) fn xthl(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let h = state.cpu.h;
    let l = state.cpu.l;
    let sp = state.cpu.sp;
    state.cpu.l = state.read(sp)?;
    state.cpu.h = state.read(sp.wrapping_add(1))?;

    state.write(sp, l)?;
    state.write(sp.wrapping_add(1), h)?;
    Ok(18)
}

pub(crate) fn cmc(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    state.cpu.cc.cy = int_bool(state.cpu.cc.cy) ^ 1 == 1;
    Ok(4)
}
pub(crate) fn ldax(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    match reg {
        B => {
            let b = state.cpu.b;
            let c = state.cpu.c;
            state.cpu.a = state.read(to_adr(b, c))?;
        }
        D => {
            let d = state.cpu.d;
            let e = state.cpu.e;
            state.cpu.a = state.read(to_adr(d, e))?;
        }
        s => unimplemented!("unimplemented lxi: {:?}", s),
    };
    Ok(7)
}

pub(crate) fn stax(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    match &reg {
        D => {
            let a = state.cpu.a;
            let adr = to_adr(state.cpu.d, state.cpu.e);
            state.write(adr, a)?;
        }
        B => {
            let a = state.cpu.a;
            let adr = to_adr(state.cpu.b, state.cpu.c);
            state.write(adr, a)?;
        }
        r => return Err(ErrorKind::OpError(format!("illegal stax: register {:?}", *r)).into()),
    };
    Ok(7)
}

pub(crate) fn mov(reg: Register, reg2: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let mut cycles = 5;
    match (&reg, &reg2) {
        (M, r) => {
            cycles = 7;
            let offset = to_adr(state.cpu.h, state.cpu.l);
            let val = state.get_u8(*r);
            state.write(offset, val)?;
        }

        (r, M) => {
            cycles = 7;
            let offset = to_adr(state.cpu.h, state.cpu.l);
            let val = state.read(offset)?;
            state.set_u8(*r, val)
        }
        (r1, r2) => {
            let data = state.get_u8(*r2);
            state.set_u8(*r1, data);
        }
    };
    Ok(cycles)
}

pub(crate) fn mvi(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let mut cycles = 7;
    match &reg {
        SP | PSW => {
            unimplemented!("unimplemented mvi: {:?}", reg);
        }

        M => {
            cycles = 10;
            let h = state.cpu.h;
            let l = state.cpu.l;
            let data = state.read_1()?;
            state.write(to_adr(h, l), data)?
        }

        r => {
            let l = state.read_1()?;
            state.set_u8(*r, l);
        }
    };
    Ok(cycles)
}

pub(crate) fn push(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let sp = state.cpu.sp;
    match &reg {
        B => {
            let c = state.cpu.c;
            let b = state.cpu.b;
            state.write(sp.wrapping_sub(2), c)?;
            state.write(sp.wrapping_sub(1), b)?;
        }
        D => {
            let e = state.cpu.e;
            let d = state.cpu.d;
            state.write(sp.wrapping_sub(2), e)?;
            state.write(sp.wrapping_sub(1), d)?;
        }
        H => {
            let l = state.cpu.l;
            let h = state.cpu.h;
            state.write(sp.wrapping_sub(2), l)?;
            state.write(sp.wrapping_sub(1), h)?;
        }
        PSW => {
            let a = state.cpu.a;
            let psw = int_bool(state.cpu.cc.z)
                | int_bool(state.cpu.cc.s) << 1
                | int_bool(state.cpu.cc.p) << 2
                | int_bool(state.cpu.cc.cy) << 3
                | int_bool(state.cpu.cc.ac) << 4;
            state.write(sp.wrapping_sub(1), a)?;
            state.write(sp.wrapping_sub(2), psw)?;
        }
        _ => unimplemented!("unimplemented inx: {:?}", reg),
    };
    state.cpu.sp = sp.wrapping_sub(2);
    Ok(11)
}

fn int_bool(b: bool) -> u8 {
    if b {
        1
    } else {
        0
    }
}

pub(crate) fn sphl(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let h = state.cpu.h;
    let l = state.cpu.l;
    state.cpu.sp = to_adr(h, l);
    Ok(5)
}

pub(crate) fn pchl(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let h = state.cpu.h;
    let l = state.cpu.l;
    state.cpu.pc = to_adr(h, l);
    Ok(5)
}

pub(crate) fn pop(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let sp = state.cpu.sp;
    match &reg {
        B => {
            state.cpu.b = state.read(sp.wrapping_add(1))?;
            state.cpu.c = state.read(sp)?;
        }
        D => {
            state.cpu.d = state.read(sp.wrapping_add(1))?;
            state.cpu.e = state.read(sp)?;
        }
        H => {
            state.cpu.h = state.read(sp.wrapping_add(1))?;
            state.cpu.l = state.read(sp)?;
        }
        PSW => {
            let sp = state.cpu.sp;
            state.cpu.a = state.read(sp.wrapping_add(1))?;
            let psw = state.read(sp)?;
            state.cpu.cc.z = 0x01 == (psw & 0x01);
            state.cpu.cc.s = 0x02 == (psw & 0x02);
            state.cpu.cc.p = 0x04 == (psw & 0x04);
            state.cpu.cc.cy = 0x08 == (psw & 0x08);
            state.cpu.cc.ac = 0x10 == (psw & 0x10);
        }
        _ => unimplemented!("unimplemented inx: {:?}", reg),
    };
    state.cpu.sp = sp.wrapping_add(2);
    Ok(10)
}

pub(crate) fn log<F: Fn(u8, u8) -> u8>(
    reg: Register,
    state: &mut CPUInterface,
    cycles: u8,
    op: F,
) -> OpResult {
    state.advance()?;
    let answer = match &reg {
        SP | PSW => {
            unimplemented!("unimplemented tmp: {:?}", reg);
        }

        M => op(state.cpu.a, read_hl(state)?),

        r => op(state.cpu.a, state.get_u8(*r)),
    };
    state.cpu.a = answer;
    state.cpu.cc.logic_flags(answer);
    Ok(cycles)
}

pub(crate) fn logi<F: Fn(u8, u8) -> u8>(state: &mut CPUInterface, cycles: u8, op: F) -> OpResult {
    state.advance()?;
    let val = state.read_1()?;
    let answer = op(state.cpu.a, val);
    state.cpu.a = answer;
    state.cpu.cc.logic_flags(answer);
    Ok(cycles)
}

pub(crate) fn cpi(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let immediate = state.read_1()?;
    let a = state.cpu.a;
    let x = a.wrapping_sub(immediate);
    state.cpu.cc.flags_zsp(x);
    state.cpu.cc.cy = a < immediate;
    Ok(7)
}

pub(crate) fn cmp(reg: Register, state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let a: u16 = state.cpu.a.into();
    let mut cycles = 4;
    let x = match &reg {
        SP | PSW => {
            unimplemented!("unimplemented tmp: {:?}", reg);
        }

        M => {
            cycles = 7;
            let val = read_hl(state)?;
            a.wrapping_sub(val.into())
        }
        r => {
            let val = state.get_u8(*r);
            a.wrapping_sub(val.into())
        }
    };
    state.cpu.cc.arith_flags(x);
    Ok(cycles)
}

pub(crate) fn ret_if<F: Fn(&CPUInterface) -> bool>(state: &mut CPUInterface, cond: F) -> OpResult {
    state.advance()?;
    if cond(state) {
        let sp = state.cpu.sp;
        let l = state.read(sp)?;
        let h = state.read(sp.wrapping_add(1))?;
        state.cpu.pc = to_adr(h, l);
        state.cpu.sp = sp.wrapping_add(2);
        Ok(11)
    } else {
        Ok(5)
    }
}

pub(crate) fn jmp_if<F: Fn(&CPUInterface) -> bool>(state: &mut CPUInterface, f: F) -> OpResult {
    state.advance()?;
    if f(state) {
        let l = state.read_1()?;
        let h = state.read_1()?;

        if state.cpu.debug && l == 0x0 && h == 0x0 {
            Err(ErrorKind::Exit(0))?
        }
        state.cpu.pc = to_adr(h, l);
    } else {
        state.cpu.pc += 2;
    }
    Ok(10)
}

pub(crate) fn call_if<F: Fn(&CPUInterface) -> bool>(state: &mut CPUInterface, cond: F) -> OpResult {
    if cond(state) {
        call(state)
    } else {
        state.cpu.pc += 3;
        Ok(11)
    }
}

pub(crate) fn call(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let l = state.read_1()?;
    let h = state.read_1()?;

    if state.cpu.debug && 5 == (u16::from(h) << 8 | u16::from(l)) {
        if state.cpu.c == 9 {
            let mut offset = (u16::from(state.cpu.d) << 8 | u16::from(state.cpu.e)) + 3;
            let mut buf = String::new();
            while let Ok(s) = state.read(offset) {
                if s == b'$' {
                    break;
                }
                buf.push(s.into());

                offset += 1;
            }
            print!("{}", buf);
        } else if state.cpu.c == 2 {
            //saw this in the inspected code, never saw it called
            print!("{:#X?}", state.cpu.e.to_ascii_uppercase());
        }
        Ok(17)
    } else if state.cpu.debug && 0 == (u16::from(h) << 8) | u16::from(l) {
        Err(ErrorKind::Exit(0).into())
    } else {
        let ret = state.cpu.pc;
        let sp = state.cpu.sp;
        let sp2 = sp.wrapping_sub(2);

        state.cpu.pc = to_adr(h, l);

        state.write(sp.wrapping_sub(1), ((ret >> 8) & 0xff) as u8)?;
        state.write(sp2, (ret & 0xff) as u8)?;
        state.cpu.sp = sp2 as u16;

        Ok(17)
    }
}

fn to_adr(h: u8, l: u8) -> u16 {
    ((u16::from(h)) << 8 | u16::from(l))
}

fn read_2_address(state: &mut CPUInterface) -> Result<u16, Error> {
    let l: u16 = state.read_1()?.into();
    let h: u16 = state.read_1()?.into();
    Ok(h << 8 | l)
}

fn read_hl(state: &mut CPUInterface) -> Result<u8, Error> {
    let h = state.cpu.h;
    let l = state.cpu.l;
    state.read(to_adr(h, l))
}

fn write_hl(state: &mut CPUInterface, data: u8) -> Result<(), Error> {
    let h = state.cpu.h;
    let l = state.cpu.l;
    state.write(to_adr(h, l), data)
}

pub(crate) fn rlc(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let x = state.cpu.a;
    state.cpu.a = (x << 1) | ((x & 0x80) >> 7);
    state.cpu.cc.cy = 128 == (x & 128);
    Ok(4)
}

pub(crate) fn ral(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let x = state.cpu.a;
    let carry = if state.cpu.cc.cy { 1 } else { 0 };
    state.cpu.a = carry | (x << 1);
    state.cpu.cc.cy = 128 == (x & 128);
    Ok(4)
}

pub(crate) fn rrc(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let x = state.cpu.a;
    state.cpu.a = ((x & 1) << 7) | (x >> 1);
    state.cpu.cc.cy = 1 == (x & 1);
    Ok(4)
}

pub(crate) fn rar(state: &mut CPUInterface) -> OpResult {
    state.advance()?;
    let x = state.cpu.a;
    let carry = if state.cpu.cc.cy { 1 } else { 0 };
    state.cpu.a = (carry << 7) | (x >> 1);
    state.cpu.cc.cy = 1 == (x & 1);
    Ok(4)
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::machine::cpu::*;
    use crate::machine::memory::Memory;
    use std::sync::RwLock;

    #[test]
    fn test_rlc() {
        let cpu = RwLock::new(new());
        let memory = RwLock::new(Memory::new(vec![0x0, 0x0]));
        let mut interface = CPUInterface {
            memory: &mut *memory.write().unwrap(),
            cpu: &mut *cpu.write().unwrap(),
        };
        interface.cpu.a = 0x0F2;
        rlc(&mut interface).unwrap();
        assert_eq!(interface.cpu.a, 0x0E5);
        assert_eq!(interface.cpu.cc.cy, true);
    }

    #[test]
    fn test_ral() {
        let cpu = RwLock::new(new());
        let memory = RwLock::new(Memory::new(vec![0x0, 0x0]));
        let mut interface = CPUInterface {
            memory: &mut *memory.write().unwrap(),
            cpu: &mut *cpu.write().unwrap(),
        };
        interface.cpu.a = 0x0B5;
        ral(&mut interface).unwrap();
        assert_eq!(interface.cpu.a, 0x06a);
        assert_eq!(interface.cpu.cc.cy, true);
    }

}
