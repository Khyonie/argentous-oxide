#![allow(dead_code, unused_variables)]

use crate::component::cpu::{Cpu, Flag, Register};

pub fn inst_len1(cpu: &mut Cpu, opcode: u8) -> fn(&mut Cpu, u8) -> Option<u8>
{
    match opcode
    {
        0x00 => return | cpu, opcode | nop(cpu, opcode),
        0xAF => return | cpu, opcode | xor_a(cpu, opcode),
        0xF3 => return | cpu, opcode | di(cpu, opcode),
        0x47 => return | cpu, opcode | ld_b_a(cpu, opcode),
        0x2F => return | cpu, opcode | cpl(cpu, opcode),
        _ => {
            return | cpu, opcode | bad_opcode1(cpu, opcode);
        }
    }
}

pub fn inst_len2(cpu: &mut Cpu, opcode: u8, arg: u8) -> fn(&mut Cpu, u8, u8) -> Option<u8>
{
    match opcode
    {
        0xFE => return | cpu, opcode, arg | cp_a(cpu, opcode, arg),
        0x28 => return | cpu, opcode, arg | jr_z(cpu, opcode, arg),
        0x18 => return | cpu, opcode, arg | jr(cpu, opcode, arg),
        0x20 => return | cpu, opcode, arg | jr_nz(cpu, opcode, arg),
        0xE0 => return | cpu, opcode, arg | ld_a8_a(cpu, opcode, arg),
        0x3E => return | cpu, opcode, arg | ld_a_u8(cpu, opcode, arg),
        0xE6 => return | cpu, opcode, arg | and_u8(cpu, opcode, arg),
        0xF0 => return | cpu, opcode, arg | ld_a_a(cpu, opcode, arg),
        0xCB => match_prefixed_opcode(cpu, 0xCB, arg),
        _ => {
            return | cpu, opcode, arg | bad_opcode2(cpu, opcode, arg);
        }
    }
}

pub fn inst_len3(cpu: &mut Cpu, opcode: u8, low: u8, high: u8) -> fn(&mut Cpu, u8, u8, u8) -> Option<u8>
{
    match opcode
    {
        0xC3 => return | cpu, opcode, low, high | jp(cpu, opcode, low, high),
        0xEA => return | cpu, opcode, low, high | ld_a(cpu, opcode, low, high),
        0xCD => return | cpu, opcode, low, high | call(cpu, opcode, low, high),
        _ => {
            return | cpu, opcode, low, high | bad_opcode3(cpu, opcode, low, high);
        }
    }
}

// The grand table

fn bad_opcode1(cpu: &mut Cpu, opcode: u8) -> Option<u8>
{
    Some(3)
}

fn bad_opcode2(cpu: &mut Cpu, opcode: u8, arg: u8) -> Option<u8>
{
    Some(3)
}

fn bad_opcode3(cpu: &mut Cpu, opcode: u8, low: u8, high: u8) -> Option<u8>
{
    Some(3)
}

/// 0x00 NO OPERATION (NOP)
fn nop(cpu: &mut Cpu, opcode: u8) -> Option<u8>
{
    // NO OPERATION
    None
}

/// 0x18 JR
fn jr(cpu: &mut Cpu, opcode: u8, arg: u8) -> Option<u8>
{
    cpu.registers.pc += arg as u16;

    None
}

fn jp(cpu: &mut Cpu, opcode: u8, low: u8, high: u8) -> Option<u8>
{
    cpu.registers.pc = ((low as u16) | ((high as u16) << 8)) - 1;

    None
}

fn xor_a(cpu: &mut Cpu, opcode: u8) -> Option<u8>
{
    cpu.registers.af ^= cpu.registers.af & 0xFF00;

    cpu.set_multi_flags(
        (cpu.registers.af & 0xFF00 == 0) as i8, 
        0, 
        0, 
        0
    );

    None
}

/// Take the logical AND of register A and the given immediate, and store the result in A
/// Sets Z if A == 0, sets N, H, and C as 0, 1, 0 respectively.
fn and_u8(cpu: &mut Cpu, opcode: u8, arg: u8) -> Option<u8>
{
    cpu.registers.af &= ((arg as u16) << 8) | 0x00FF;

    cpu.set_multi_flags(
        (cpu.registers.af >> 8 == 0) as i8, 
        0, 
        1, 
        0
    );

    None
}

fn ld_a(cpu: &mut Cpu, opcode: u8, low: u8, high: u8) -> Option<u8>
{
    // TODO For now treat all memory as read/write capable, this is bad.
    cpu.memory[(((high as u16) << 8) | low as u16) as usize] = (cpu.registers.af >> 8) as u8;

    None
}

fn ld_a8_a(cpu: &mut Cpu, opcode: u8, arg: u8) -> Option<u8>
{
    cpu.memory[(0xFF00 | arg as u16) as usize] = (cpu.registers.af >> 8) as u8;

    None
}

fn ld_a_u8(cpu: &mut Cpu, opcode: u8, arg: u8) -> Option<u8>
{
    cpu.registers.af |= (cpu.registers.af & 0x00FF) | ((arg as u16) << 8); 

    None
}

fn ld_a_a(cpu: &mut Cpu, opcode: u8, arg: u8) -> Option<u8>
{
    cpu.registers.af = ((cpu.memory[((arg as u16) << 8 | 0x00FF) as usize] as u16) << 8) | (cpu.registers.af & 0x00FF);

    None
}

fn ld_b_a(cpu: &mut Cpu, opcode: u8) -> Option<u8>
{
    cpu.load_inter_register(Register::B, Register::A);

    None
}

fn di(cpu: &mut Cpu, opcode: u8) -> Option<u8>
{
    // TODO This
    None
}

fn cpl(cpu: &mut Cpu, opcode: u8) -> Option<u8>
{
    cpu.registers.af = (!((cpu.registers.af >> 8) as u8) as u16) << 8 | (cpu.registers.af & 0x00FF);

    None
}

fn cp_a(cpu: &mut Cpu, opcode: u8, arg: u8) -> Option<u8>
{
    // TODO Run operation once and store the result

    cpu.set_multi_flags(
        ((cpu.registers.af >> 8) as u8 == arg) as i8, 
        1, 
        (((cpu.registers.af >> 8) as u8 == arg) && (((cpu.registers.af >> 8) as u8) == 0x0F)) as i8, 
        (arg >= ((cpu.registers.af >> 8) as u8)) as i8
    );

    None
}

fn jr_z(cpu: &mut Cpu, opcode: u8, arg: u8) -> Option<u8>
{
    if cpu.get_flag(Flag::Z)
    {
        cpu.registers.pc += arg as u16;
    }

    None
}

fn jr_nz(cpu: &mut Cpu, opcode: u8, arg: u8) -> Option<u8>
{
    if !cpu.get_flag(Flag::Z)
    {
        cpu.registers.pc += arg as u16;
    }

    None
}

fn call(cpu: &mut Cpu, opcode: u8, low: u8, high: u8) -> Option<u8>
{
    cpu.push_stack((cpu.registers.pc >> 8) as u8);
    cpu.push_stack(cpu.registers.pc as u8);

    cpu.registers.pc = (((high as u16) << 8) | low as u16) - 1;

    None
}

// Prefixed opcodes (0xCB__)

fn match_prefixed_opcode(cpu: &mut Cpu, prefix: u8, ext_opcode: u8) -> fn (&mut Cpu, u8, u8) -> Option<u8>
{
    match ext_opcode
    {
        0x87 => | cpu, prefix, ext_opcode | cb_rst_0_a(cpu, prefix, ext_opcode),
        _ => | cpu, prefix, ext_opcode | bad_prefixed_opcode(cpu, prefix, ext_opcode),
    }
}

fn bad_prefixed_opcode(cpu: &mut Cpu, prefix: u8, ext_opcode: u8) -> Option<u8>
{
    Some(6)
}

fn cb_rst_0_a(cpu: &mut Cpu, prefix: u8, ext_opcode: u8) -> Option<u8>
{
    cpu.registers.af &= 0b1111_1110_1111_1111;

    None
}