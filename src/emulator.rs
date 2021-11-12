#![allow(dead_code)]

use std::num::Wrapping;

use crate::{component::{cartridge::Cartridge, cpu::Cpu}, lookups};

/// Logo bytes
const LOGO_DUMP: [u8; 48] = [0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E];

pub struct Gameboy
{
    pub cartridge: Option<Cartridge>,
    cpu: Cpu
}

impl Gameboy
{
    pub fn construct() -> Self
    {
        Gameboy
        {
            cartridge: None,
            cpu: Cpu::new()
        }
    }

    pub fn insert_cartridge(&mut self, cart: Cartridge)
    {
        self.cartridge = Some(cart);
    }

    pub fn read_cart_data(&self)
    {
        match &self.cartridge
        {
            Some(_) => { },
            None => {
                println!("Cannot read cartridge, as one is not inserted!");
                return;
            }
        }

        let checksum: u8 = self.compute_checksum();
        let cart_checksum = self.cartridge.as_ref().unwrap().rom[0x14D];

        if cart_checksum != checksum
        {
            println!("Header checksum failed. This may mean the ROM is corrupt. (Expected: 0x{:0>2X}, given: 0x{:0>2X})", cart_checksum, checksum);
        } else {
            println!("Header checksum succeeded. (Expected: 0x{:0>2X})", checksum);
        }

        let global_checksum: u16 = self.compute_dumb_checksum();
        let cart_global_checksum: u16 = self.cartridge.as_ref().unwrap().rom[0x14F] as u16 | ((self.cartridge.as_ref().unwrap().rom[0x14E] as u16) << 8);

        if global_checksum != cart_global_checksum
        {
            println!("Global checksum failed. This is usually safe to ignore.");
        } else {
            println!("Global checksum succeeded.");
        }

        // Read off header bytes

        // Title
        let name: &str = &self.cart().rom[0x134..=0x143] // Take the bytes that hold the title,
            .iter() // turn it into an iterator,
            .map(| n | *n as char) // map the bytes to chars,
            .collect::<String>(); // and collect it into a String

        println!("\nCartridge title: {}", name);

        // Logo
        let logo: [u8; 48] = self.cart().rom[0x104..=0x133].try_into().unwrap();
        if logo != LOGO_DUMP
        {
            println!("Invalid Nintendo logo. This may mean the ROM is corrupt, or it is an unofficial cartridge.");
        } else {
            println!("Valid Nintendo logo found.");
        }

        let cart_meta = lookups::mbc_type(self.cart().rom[0x147]);
        println!("Cartridge meta: Type: {} | No. of ROM banks: {} | No. of RAM banks: {} | Battery backed: {} | Timer present: {}", cart_meta.0, cart_meta.1, cart_meta.2, cart_meta.3, cart_meta.4);

        // ROM size
        println!("ROM size: {}KiB", 32 << self.cart().rom[0x148]);

        // RAM size
        println!("RAM size: {}KiB", match self.cart().rom[0x149] {
            0 => {
                "0"
            },
            2 => {
                "8"
            },
            3 => {
                "32"
            },
            4 => {
                "128"
            },
            5 => {
                "64"
            },
            _ => {
                "Unknown"
            }
        });
    }

    pub fn compute_checksum(&self) -> u8
    {
        let mut x: Wrapping<u8> = Wrapping(0);

        for i in self.cartridge.as_ref().unwrap().rom[0x134..=0x14C].iter()
        {
            x = x - Wrapping(*i) - Wrapping(1);
        }

        x.0
    }

    /// Computes the cartridge's global checksum. The gameboy doesn't actually do this.
    pub fn compute_dumb_checksum(&self) -> u16
    {
        let mut x: Wrapping<u16> = Wrapping(0);

        for (i, e) in self.cartridge.as_ref().unwrap().rom[0x0000..=0xFFFF].iter().enumerate()
        {
            if i == 0x14E || i == 0x14F
            {
                continue;
            }

            x = x + Wrapping(*e as u16);
        }

        x.0
    }

    pub fn cart(&self) -> &Cartridge
    {
        self.cartridge.as_ref().unwrap()
    }

    pub fn start_cart(&mut self, print_state: bool)
    {
        // Init PC
        // Begin execution at $0100
        self.cpu.registers.pc = 0x0100;

        let mut opcode;
        let mut len;

        if print_state
        {
            println!("----------< BEGIN READOUT >----------");
            println!("ADDRESS   | CACHE: INST ARGL ARGH | REGISTERS: AF:   HHLL  BC:   HHLL  DE:   HHLL  HL:   HHLL");
        }

        let exitcode = loop {
            self.cpu.cache.clear();
            self.cpu.registers.pc += 1;

            opcode = self.cartridge.as_ref().unwrap().rom[self.cpu.registers.pc as usize];
            len = lookups::instruction_len(&opcode);

            // Load bytes into cache
            while self.cpu.cache.len() < (len - 1) as usize
            {
                self.cpu.registers.pc += 1;
                self.cpu.cache.push(self.cartridge.as_ref().unwrap().rom[self.cpu.registers.pc as usize]);
            }

            if print_state
            {
                self.cpu.print_state(opcode);
            }

            match self.cpu.execute(opcode)
            {
                Some(code) => break code,
                None => {
                    continue;
                },
            }
        };
        
        println!("----------<  END READOUT  >----------");

        println!("Gameboy routine exited with exit code {} ({} | Opcode 0x{:0>2X} @ PC ${:0>4X}).", exitcode, lookups::exit_codes(exitcode), opcode, self.cpu.registers.pc - (len as u16 - 1));
    }
}