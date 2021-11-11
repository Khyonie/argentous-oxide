#![allow(dead_code)]

pub struct Cpu
{
    pub memory: [u8; 65536],
    pub registers: Registers
}

pub struct Registers
{
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub pc: u16,
    pub sp: u16
}

impl Registers
{
    pub fn new() -> Self
    {
        Registers 
        { 
            af: 0, 
            bc: 0, 
            de: 0, 
            hl: 0, 
            pc: 0, 
            sp: 0 
        }
    }
}

impl Cpu
{
    pub fn new() -> Self
    {
        Cpu 
        { 
            memory: [0; 65536], 
            registers: Registers::new()
        }
    }
}