pub struct Cpu
{
    memory: [u8; 65536],
    registers: Registers
}

pub struct Registers
{
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    pc: u16,
    sp: u16
}

impl Cpu
{
    
}