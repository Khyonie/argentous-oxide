#![allow(dead_code)]

use crate::{cpu::operations, lookups};

pub struct Cpu
{
    pub memory: [u8; 65536],
    pub registers: Registers,
    pub cache: Vec<u8>,
}

pub enum Flag
{
    Z = 7,
    N = 6,
    H = 5,
    C = 4
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

pub enum Register
{
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L
}

impl Registers
{
    pub fn new() -> Self
    {
        Registers 
        { 
            af: 0x0100, 
            bc: 0x0014, 
            de: 0x0000, 
            hl: 0xC060, 
            pc: 0x00FF, 
            sp: 0xFFFE 
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

    pub fn set_flag(&mut self, flag: Flag)
    {
        self.registers.af |= (0b0000_0001u8 << flag as u8) as u16;
    }

    pub fn clear_flag(&mut self, flag: Flag)
    {
        self.registers.af &= !(0b0000_0001u8 << flag as u8) as u16;
    }

    pub fn get_flag(&self, flag: Flag) -> bool
    {
        self.registers.af & 0b0000_0001 << flag as u8 > 0
    }

    pub fn push_stack(&mut self, data: u8)
    {
        self.registers.sp -= 1;

        self.memory[self.registers.sp as usize] = data;
    }

    pub fn pop_stack(&mut self) -> u8
    {
        let data = self.memory[self.registers.sp as usize];

        if self.registers.sp != 0xFFFF
        {
            self.registers.sp += 1;
        }

        data
    }

    pub fn load_inter_register(&mut self, target: Register, source: Register)
    {
        let value: u8 = match source
        {
            Register::A => (self.registers.af >> 8) as u8,
            Register::F => self.registers.af as u8,
            Register::B => (self.registers.bc >> 8) as u8,
            Register::C => self.registers.bc as u8,
            Register::D => (self.registers.de >> 8) as u8,
            Register::E => self.registers.de as u8,
            Register::H => (self.registers.hl >> 8) as u8,
            Register::L => self.registers.hl as u8,
        };

        match target
        {
            Register::A => self.registers.af = (self.registers.af & 0x00FF) | ((value as u16) << 8),
            Register::F => self.registers.af = (self.registers.af & 0xFF00) | (value as u16),
            Register::B => self.registers.bc = (self.registers.bc & 0x00FF) | ((value as u16) << 8),
            Register::C => self.registers.bc = (self.registers.bc & 0xFF00) | (value as u16),
            Register::D => self.registers.de = (self.registers.de & 0x00FF) | ((value as u16) << 8),
            Register::E => self.registers.de = (self.registers.de & 0xFF00) | (value as u16),
            Register::H => self.registers.hl = (self.registers.hl & 0x00FF) | ((value as u16) << 8),
            Register::L => self.registers.hl = (self.registers.hl & 0xFF00) | (value as u16)
        }
    }

    pub fn logical_and_register_u8(&mut self, byte: u8, register: Register)
    {
        match register
        {
            Register::A => self.registers.af &= ((byte as u16) << 8) | 0x00FF,
            Register::F => self.registers.af &= 0xFF00 | byte as u16,
            Register::B => self.registers.af &= ((byte as u16) << 8) | 0x00FF,
            Register::C => self.registers.af &= 0xFF00 | byte as u16,
            Register::D => self.registers.af &= ((byte as u16) << 8) | 0x00FF,
            Register::E => self.registers.af &= 0xFF00 | byte as u16,
            Register::H => self.registers.af &= ((byte as u16) << 8) | 0x00FF,
            Register::L => self.registers.af &= 0xFF00 | byte as u16,
        }
    }

    pub fn logical_and_inter_register(&mut self, register: Register)
    {
        // The target will always be register A
        match register
        {
            Register::A => self.logical_and_register_u8((self.registers.af >> 8) as u8, Register::A),
            Register::F => self.logical_and_register_u8(self.registers.af as u8, Register::A),
            Register::B => self.logical_and_register_u8((self.registers.bc >> 8) as u8, Register::A),
            Register::C => self.logical_and_register_u8(self.registers.bc as u8, Register::A),
            Register::D => self.logical_and_register_u8((self.registers.de >> 8) as u8, Register::A),
            Register::E => self.logical_and_register_u8(self.registers.de as u8, Register::A),
            Register::H => self.logical_and_register_u8((self.registers.hl >> 8) as u8, Register::A),
            Register::L => self.logical_and_register_u8(self.registers.hl as u8, Register::A),
        }
    }

    /// Sets, clears, or preserves all flags. 0 = clear, 1 = set, anything else preserves.
    pub fn set_multi_flags(&mut self, z: i8, n: i8, h: i8, c: i8)
    {
        match z
        {
            0 => self.clear_flag(Flag::Z),
            1 => self.set_flag(Flag::Z),
            _ => { }
        }

        match n
        {
            0 => self.clear_flag(Flag::N),
            1 => self.set_flag(Flag::N),
            _ => { }
        }

        match h
        {
            0 => self.clear_flag(Flag::H),
            1 => self.set_flag(Flag::H),
            _ => { }
        }

        match c
        {
            0 => self.clear_flag(Flag::C),
            1 => self.set_flag(Flag::C),
            _ => { }
        }
    }

    pub fn print_state(&self, opcode: u8)
    {
        println!("PC: ${:0>4X} | Cache: {: <6} {} {} | Registers: AF: 0x{:0>4X}, BC: 0x{:0>4X}, DE: 0x{:0>4X}: HL: 0x{:0>4X} | Flags: Z: [{}], N: [{}], H: [{}], C: [{}]", 
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
            self.registers.hl,
            match self.get_flag(Flag::Z) { true => "X", false => " " },
            match self.get_flag(Flag::N) { true => "X", false => " " },
            match self.get_flag(Flag::H) { true => "X", false => " " },
            match self.get_flag(Flag::C) { true => "X", false => " " },
        );
    }
}