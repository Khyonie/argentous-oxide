#![allow(dead_code, unused_variables)]

use crate::component::cpu::Cpu;

pub fn inst_len1(cpu: &mut Cpu, opcode: u8) -> fn(&mut Cpu, u8) -> Option<u8>
{
    match opcode
    {
        _ => {
            return | cpu, opcode | bad_opcode1(cpu, opcode);
        }
    }
}

pub fn inst_len2(cpu: &mut Cpu, opcode: u8, low: u8) -> fn(&mut Cpu, u8, u8) -> Option<u8>
{
    match opcode
    {
        _ => {
            return | cpu, opcode, low | bad_opcode2(cpu, opcode, low);
        }
    }
}

pub fn inst_len3(cpu: &mut Cpu, opcode: u8, low: u8, high: u8) -> fn(&mut Cpu, u8, u8, u8) -> Option<u8>
{
    match opcode
    {
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