use emulator::emulate::pause;
use emulator::state::State;
use ops::Register;
use ops::Register::*;

pub(crate) fn add(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    let answer = match reg {
        Register::M => {
            let offset = ((state.h as u16) << 8) | state.l as u16;
            let m = state.read(offset)? as u16;
            (state.a as u16) + m
        }
        r => (state.a as u16) + (state.get_u8(r) as u16),
    };
    state.cc.arith_flags(answer);
    state.a = (answer & 0xff) as u8;
    Ok(())
}

pub(crate) fn sub(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    let answer = match reg {
        Register::M => {
            let offset = ((state.h as u16) << 8) | state.l as u16;
            let m = state.read(offset)? as u16;
            (state.a as i16 - m as i16) as u16
        }
        r => ((state.a as i16) - (state.get_u8(r) as i16)) as u16,
    };
    state.cc.arith_flags(answer);
    state.a = (answer & 0xff) as u8;
    Ok(())
}

pub(crate) fn aci(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let db = state.read_1()? as u16;
    let carry = if state.cc.cy { 1 } else { 0 };
    let a = state.a as u16;
    let result = a.wrapping_add(db + carry);
    state.a = (result & 0xff) as u8;
    state.cc.arith_flags(result);
    Ok(())
}

pub(crate) fn sui(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let db = state.read_1()? as u16;
    let a = state.a as u16;
    let result = a.wrapping_sub(db);
    state.a = (result & 0xff) as u8;
    state.cc.arith_flags(result);
    Ok(())
}

pub(crate) fn sbi(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let db = state.read_1()? as u16;
    let carry = if state.cc.cy { 1 } else { 0 };
    let a = state.a as u16;
    let result = a.wrapping_sub(db + carry);
    state.a = (result & 0xff) as u8;
    state.cc.arith_flags(result);
    Ok(())
}
pub(crate) fn sbb(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    let carry = if state.cc.cy { 1 } else { 0 };
    let answer = match reg {
        Register::M => {
            let offset = to_adr(state.h, state.l);
            let m = state.read(offset)? + carry;

            (state.a as i16 - m as i16) as u16
        }
        r => (state.a as i16 - (state.get_u8(r) as i16 + (carry as i16))) as u16,
    };

    state.cc.arith_flags(answer);
    state.a = (answer & 0xff) as u8;
    Ok(())
}

pub(crate) fn adi(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let val = state.read_1()?;
    let answer: u16 = (state.a as u16) + (val as u16);

    state.cc.arith_flags(answer);
    state.a = (answer & 0xff) as u8;
    Ok(())
}

pub(crate) fn inr(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    let answer = match reg {
        Register::M => {
            let offset = ((state.h as u16) << 8) | state.l as u16;
            let m = state.read(offset)? as u16;
            let result = m + 1;
            write_hl(state, (result & 0xff) as u8)?;
            result as u16
        }
        r => {
            let result = state.get_u8(r) + 1;
            state.set_u8(reg, (result & 0xff) as u8);
            result as u16
        }
    };
    state.cc.arith_flags(answer);
    Ok(())
}

pub(crate) fn ani(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let data = state.read_1()? as u16;
    let answer = (state.a as u16) & data;

    state.cc.arith_flags(answer);
    state.a = (answer & 0xff) as u8;
    Ok(())
}

pub(crate) fn lxi(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    match reg {
        SP => {
            let l = state.read_1()? as u16;
            let h = state.read_1()? as u16;
            state.sp = h << 8 | l;
        }
        B => {
            state.c = state.read_1()?;
            state.b = state.read_1()?;
        }
        D => {
            state.e = state.read_1()?;
            state.d = state.read_1()?;
        }
        H => {
            state.l = state.read_1()?;
            state.h = state.read_1()?;
        }
        s => unimplemented!("unimplemented lxi: {:?}", s),
    };
    Ok(())
}

pub(crate) fn dad(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    let answer = match reg {
        SP => {
            let hl = to_adr(state.h, state.l);
            hl.wrapping_add(state.sp)
        }
        B => {
            let bc = to_adr(state.b, state.c);
            let hl = to_adr(state.h, state.l);
            hl.wrapping_add(bc)
        }
        D => {
            let bc = to_adr(state.d, state.e);
            let hl = to_adr(state.h, state.l);
            hl.wrapping_add(bc)
        }
        H => {
            let hl = to_adr(state.h, state.l);
            hl.wrapping_add(hl)
        }
        s => unimplemented!("unimplemented lxi: {:?}", s),
    };

    state.cc.carry(answer);

    let (h, l) = split_u16(answer);
    state.h = h;
    state.l = l;
    Ok(())
}

pub(crate) fn lda(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let adr = read_2_address(state)?;
    Ok(state.a = state.read(adr)?)
}

pub(crate) fn sta(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let a = state.a;
    let adr = read_2_address(state)?;
    state.write(adr, a)
}

// 	L <- (adr); H<-(adr+1)
pub(crate) fn lhld(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let l = state.read_1()?;
    let h = state.read_1()?;
    let adr = to_adr(h, l);
    state.l = state.read(adr)?;
    state.h = state.read((adr + 1))?;
    Ok(())
}

// 	(adr) <-L; (adr+1)<-H
pub(crate) fn shld(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let l = state.read_1()?;
    let h = state.read_1()?;
    let adr = to_adr(h, l);
    state.write(adr, l)?;
    state.write(adr.wrapping_add(1), h)?;
    Ok(())
}

pub(crate) fn xchg(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let d = state.d;
    let e = state.e;
    state.d = state.h;
    state.e = state.l;
    state.h = d;
    state.l = e;
    Ok(())
}

pub(crate) fn xthl(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let h = state.h;
    let l = state.l;
    let sp = state.sp;
    state.l = state.read(sp)?;
    state.h = state.read(sp.wrapping_add(1))?;

    state.write(sp, l)?;
    state.write(sp.wrapping_add(1), h)?;
    Ok(())
}

pub(crate) fn cmc(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let cy = if int_bool(state.cc.cy) ^ 1 == 1 {
        true
    } else {
        false
    };
    state.cc.cy = cy;
    Ok(())
}
pub(crate) fn ldax(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    match reg {
        B => {
            let b = state.b;
            let c = state.c;
            state.a = state.read(to_adr(b, c))?;
        }
        D => {
            let d = state.d;
            let e = state.e;
            state.a = state.read(to_adr(d, e))?;
        }
        s => unimplemented!("unimplemented lxi: {:?}", s),
    };
    Ok(())
}

pub(crate) fn stax(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    match &reg {
        D => {
            let a = state.a;
            let adr = to_adr(state.d, state.e);
            state.write(adr, a)?;
        }
        B => {
            let a = state.a;
            let adr = to_adr(state.b, state.c);
            state.write(adr, a)?;
        }
        r => return Err(format!("illegal stax: register {:?}", *r)),
    };
    Ok(())
}

pub(crate) fn mov(reg: Register, reg2: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    match (&reg, &reg2) {
        (M, r) => {
            let offset = to_adr(state.h, state.l);
            let val = state.get_u8(*r);
            state.write(offset, val)?;
        }

        (r, M) => {
            let offset = to_adr(state.h, state.l);
            let val = state.read(offset)?;
            state.set_u8(*r, val)
        }
        (r1, r2) => {
            let data = state.get_u8(*r2);
            state.set_u8(*r1, data);
        }
    };
    Ok(())
}

pub(crate) fn mvi(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    match &reg {
        SP | PSW => {
            unimplemented!("unimplemented mvi: {:?}", reg);
        }

        M => {
            let h = state.h;
            let l = state.l;
            let data = state.read_1()?;
            state.write(to_adr(h, l), data)?
        }

        r => {
            let l = state.read_1()?;
            state.set_u8(*r, l);
        }
    };
    Ok(())
}

pub(crate) fn inx(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    match &reg {
        B => {
            state.c = state.c.wrapping_add(1);
            if state.c == 0 {
                state.b += 1;
            }
        }
        D => {
            state.e = state.e.wrapping_add(1);
            if state.e == 0 {
                state.d += 1;
            }
        }
        SP => {
            state.sp = state.sp.wrapping_add(1);
        }
        H => {
            state.l = state.l.wrapping_add(1);
            if state.l == 0 {
                state.h += 1;
            }
        }
        _ => unimplemented!("unimplemented inx: {:?}", reg),
    };
    Ok(())
}

pub(crate) fn dcx(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    match &reg {
        B => {
            state.c = state.c.wrapping_sub(1);
            if state.c == 0 {
                state.b -= 1;
            }
        }
        D => {
            state.e = state.c.wrapping_sub(1);
            if state.e == 0 {
                state.d = state.d.wrapping_sub(1);
            }
        }
        SP => {
            state.sp = state.sp.wrapping_sub(1);
        }
        H => {
            state.l = wrapping(state.l, |l| l - 1);
            if state.l == 0 {
                state.h -= 1;
            }
        }
        _ => unimplemented!("unimplemented inx: {:?}", reg),
    };
    Ok(())
}

pub(crate) fn push(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    let sp = state.sp;
    match &reg {
        B => {
            let c = state.c;
            let b = state.b;
            state.write(sp.wrapping_sub(2), c)?;
            state.write(sp.wrapping_sub(1), b)?;
        }
        D => {
            let e = state.e;
            let d = state.d;
            state.write(sp.wrapping_sub(2), e)?;
            state.write(sp.wrapping_sub(1), d)?;
        }
        H => {
            let l = state.l;
            let h = state.h;
            state.write(sp.wrapping_sub(2), l)?;
            state.write(sp.wrapping_sub(1), h)?;
        }
        PSW => {
            let a = state.a;
            let psw = int_bool(state.cc.z)
                | int_bool(state.cc.s) << 1
                | int_bool(state.cc.p) << 2
                | int_bool(state.cc.cy) << 3
                | int_bool(state.cc.ac) << 4;
            state.write(sp.wrapping_sub(1), a)?;
            state.write(sp.wrapping_sub(2), psw)?;
        }
        _ => unimplemented!("unimplemented inx: {:?}", reg),
    };
    state.sp = sp.wrapping_sub(2);
    Ok(())
}

fn int_bool(b: bool) -> u8 {
    if b {
        1
    } else {
        0
    }
}

pub(crate) fn sphl(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let h = state.h;
    let l = state.l;
    Ok(state.sp = to_adr(h, l))
}

pub(crate) fn pchl(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let h = state.h;
    let l = state.l;
    Ok(state.pc = to_adr(h, l))
}

pub(crate) fn pop(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    let sp = state.sp;
    match &reg {
        B => {
            state.b = state.read(sp.wrapping_add(1))?;
            state.c = state.read(sp)?;
        }
        D => {
            state.d = state.read(sp.wrapping_add(1))?;
            state.e = state.read(sp)?;
        }
        H => {
            state.h = state.read(sp.wrapping_add(1))?;
            state.l = state.read(sp)?;
        }
        PSW => {
            let sp = state.sp;
            state.a = state.read(sp.wrapping_add(1))?;
            let psw = state.read(sp)?;
            state.cc.z = 0x01 == (psw & 0x01);
            state.cc.s = 0x02 == (psw & 0x02);
            state.cc.p = 0x04 == (psw & 0x04);
            state.cc.cy = 0x05 == (psw & 0x08);
            state.cc.ac = 0x10 == (psw & 0x10);
        }
        _ => unimplemented!("unimplemented inx: {:?}", reg),
    };
    state.sp = sp.wrapping_add(2);
    Ok(())
}

// pub(crate) fn tmp(reg: Register, state: &mut State) -> Result<(), String> {
//    state.advance()?;
//    match &reg {
//        SP | PSW | M | H | L => {
//            unimplemented!("unimplemented tmp: {:?}", reg);
//        }
//
//        r => {
//        }
//    };
//    Ok(())
//}

pub(crate) fn log<F: Fn(u16, u16) -> u16>(
    reg: Register,
    state: &mut State,
    op: F,
) -> Result<(), String> {
    state.advance()?;
    let answer = match &reg {
        SP | PSW => {
            unimplemented!("unimplemented tmp: {:?}", reg);
        }

        M => op(state.a as u16, read_hl(state)? as u16),

        r => op(state.a as u16, state.get_u8(*r) as u16),
    };
    state.a = (answer & 0xff) as u8;
    state.cc.logic_flags(answer as u16);
    Ok(())
}

pub(crate) fn logi<F: Fn(u16, u16) -> u16>(state: &mut State, op: F) -> Result<(), String> {
    state.advance()?;
    let val = state.read_1()? as u16;
    let answer = op(state.a as u16, val);
    state.a = (answer & 0xff) as u8;
    state.cc.logic_flags(answer as u16);
    Ok(())
}

pub(crate) fn adc(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    let carry = if state.cc.cy { 1 } else { 0 };
    let answer = match reg {
        Register::M => {
            let offset = ((state.h as u16) << 8) | state.l as u16;
            let m = state.read(offset)? as u16;
            (state.a as u16) + m + carry
        }
        r => (state.a as u16) + (state.get_u8(r) as u16) + carry,
    };

    state.cc.arith_flags(answer);
    state.a = (answer & 0xff) as u8;
    Ok(())
}

pub(crate) fn cpi(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let immediate = state.read_1()? as u16;
    let a = state.a as u16;
    let x = a.wrapping_sub(immediate);
    //println!("immediate: {}, a: {}, x: {}", immediate, a, x);
    state.cc.arith_flags(x as u16);
    Ok(())
}

pub(crate) fn cmp(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    let a = state.a as i16;
    let x = match &reg {
        SP | PSW => {
            unimplemented!("unimplemented tmp: {:?}", reg);
        }

        M => {
            let val = read_hl(state)?;
            ((a - val as i16) & 0xff) as u16
        }
        r => {
            let val = state.get_u8(*r);
            ((a - val as i16) & 0xff) as u16
        }
    };
    state.cc.sign(x);
    Ok(())
}

pub(crate) fn dcr(reg: Register, state: &mut State) -> Result<(), String> {
    state.advance()?;
    let answer = match reg {
        Register::M => {
            let offset = ((state.h as u16) << 8) | state.l as u16;
            let m = state.read(offset)? as u16;
            let result = m as i16 - 1;
            write_hl(state, (result & 0xff) as u8)?;
            result as u16
        }
        r => {
            let result = ((state.get_u8(r) as i16 - 1) & 0xff) as u8;
            state.set_u8(reg, result);
            result as u16
        }
    };
    state.cc.arith_flags(answer);
    Ok(())
}

pub(crate) fn ret_if<F: Fn(&State) -> bool>(state: &mut State, cond: F) -> Result<(), String> {
    state.advance()?;
    if cond(state) {
        let sp = state.sp;
        let l = state.read(sp)?;
        let h = state.read(sp.wrapping_add(1))?;
        state.pc = (((h as u16) << 8) | l as u16);
        state.sp = sp.wrapping_add(2);
    }
    Ok(())
}

pub(crate) fn jmp_if<F: Fn(&State) -> bool>(state: &mut State, f: F) -> Result<(), String> {
    state.advance()?;
    if f(state) {
        let l = state.read_1()?;
        let h = state.read_1()?;

        if state.debug && l == 0x0 && h == 0x0 {
            Err("exit")?
        }

        let offset = (h as u16) << 8 | l as u16;
        // println!("jumping to: {:#X}", offset);
        state.pc = offset;
    } else {
        state.pc += 2;
    }
    Ok(())
}

pub(crate) fn call_if<F: Fn(&State) -> bool>(state: &mut State, cond: F) -> Result<(), String> {
    if cond(state) {
        call(state)
    } else {
        state.pc += 3;
        Ok(())
    }
}

pub(crate) fn call(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let l = state.read_1()?;
    let h = state.read_1()?;

    if state.debug && 5 == (h as u16) << 8 | l as u16 {
        if state.c == 9 {
            let mut offset = (((state.d as u16) << 8 | state.e as u16) + 3);
            let mut buf = String::new();
            while let Ok(s) = state.read(offset) {
                if s == b'$' {
                    break;
                }
                buf.push(s.into());

                offset += 1;
            }
            print!("{}", buf);
        } else if state.c == 2 {
            //saw this in the inspected code, never saw it called
            print!("{:#X?}", state.e.to_ascii_uppercase());
        }
        Ok(())
    } else if state.debug && 0 == ((h as u16) << 8) | l as u16 {
        Err("exit".to_string())
    } else {
        let ret = state.pc as u16;
        let sp = state.sp;
        let sp2 = sp.wrapping_sub(2);

        state.pc = to_adr(h, l);

        state.write(sp.wrapping_sub(1), ((ret >> 8) & 0xff) as u8)?;
        state.write(sp2, (ret & 0xff) as u8)?;
        state.sp = sp2 as u16;

        Ok(())
    }
}

fn wrapping<F: Fn(i16) -> i16>(operand: u8, op: F) -> u8 {
    (op(operand as i16) & 0xff) as u8
}

fn to_adr(h: u8, l: u8) -> u16 {
    ((h as u16) << 8 | l as u16)
}

fn split_u16(b: u16) -> (u8, u8) {
    let low = b & 0xff;
    let high = (b >> 8) & 0xff;

    (high as u8, low as u8)
}

fn read_2_address(state: &mut State) -> Result<u16, String> {
    let l = state.read_1()? as u16;
    let h = state.read_1()? as u16;
    Ok(h << 8 | l)
}

fn read_hl(state: &mut State) -> Result<u8, String> {
    let h = state.h;
    let l = state.l;
    state.read(to_adr(h, l))
}

fn write_hl(state: &mut State, data: u8) -> Result<(), String> {
    let h = state.h;
    let l = state.l;
    state.write(to_adr(h, l), data)
}

pub(crate) fn rlc(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let x = state.a;
    state.a = (x << 1) | (x >> 7);
    state.cc.cy = 1 == (x >> 7);
    Ok(())
}

pub(crate) fn ral(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let x = state.a;
    let carry = if state.cc.cy { 1 } else { 0 };
    state.a = (carry << 7) | (x << 1);
    state.cc.cy = 1 == (x >> 7);
    Ok(())
}

pub(crate) fn rrc(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let x = state.a;
    state.a = ((x & 1) << 7) | (x >> 1);
    state.cc.cy = 1 == (x & 1);
    Ok(())
}

pub(crate) fn rar(state: &mut State) -> Result<(), String> {
    state.advance()?;
    let x = state.a;
    let carry = if state.cc.cy { 1 } else { 0 };
    state.a = (carry << 7) | (x >> 1);
    state.cc.cy = 1 == (x & 1);
    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;
    use emulator::state;
    #[test]
    fn test_rlc() {
        let mut state = state::new_state(vec![0x0, 0x0]);
        state.a = 0x0F2;
        rlc(&mut state);
        assert_eq!(state.a, 0x0E5);
        assert_eq!(state.cc.cy, true);
    }

    #[test]
    fn test_ral() {
        let mut state = state::new_state(vec![0x0, 0x0]);
        state.a = 0x0B5;
        ral(&mut state);
        assert_eq!(state.a, 0x06a);
        assert_eq!(state.cc.cy, true);
    }

}
