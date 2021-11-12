#![allow(dead_code)]

use crate::{cpu::operations, lookups};

pub struct Cpu
{
    pub memory: [u8; 65536],
    pub registers: Registers,
    pub cache: Vec<u8>,
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
            registers: Registers::new(),
            cache: Vec::new()
        }
    }

    pub fn execute(&mut self, opcode: u8) -> Option<u8>
    {
        match opcode
        {
            0x10 => {
                return Some(0); // STOP opcode
            },
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => {
                return Some(4); // Unknown opcode
            },
            _ => {
                return match lookups::instruction_len(&opcode)
                {
                    1 => {
                        let inst = operations::inst_len1(self, opcode);
                        inst(self, opcode)
                    }, 
                    2 => {
                        let inst = operations::inst_len2(self, opcode, self.cache[0]);
                        inst(self, opcode, self.cache[0])
                    },
                    3 => {
                        let inst = operations::inst_len3(self, opcode, self.cache[0], self.cache[1]);
                        inst(self, opcode, self.cache[0], self.cache[1])
                    }
                    _ => {
                        panic!(); // This can't fire, but panic if it does
                    }
                }
            }
        }
    }

    pub fn print_state(&self, opcode: u8)
    {
        println!("PC: ${:0>4X} | Cache: {: <4} {} {} | Registers: AF: 0x{:0>4X}, BC: 0x{:0>4X}, DE: 0x{:0>4X}: HL: 0x{:0>4X}", 
            self.registers.pc - (lookups::instruction_len(&opcode) as u16 - 1 /* TODO Band-aid fix? */),
            lookups::grand_opcode_lookup(opcode).1,
            if self.cache.len() > 0
            {
                format!("0x{:0>2X}", self.cache[0])
            } else {
                "    ".to_string()
            },
            if self.cache.len() > 1
            {
                format!("0x{:0>2X}", self.cache[1])
            } else {
                "    ".to_string()
            },
            self.registers.af,
            self.registers.bc,
            self.registers.de,
            self.registers.hl
        );
    }
}