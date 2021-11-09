use std::{fs::read, path::Path};

/// Logo bytes
const LOGO_DUMP: [u8; 48] = [0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E];

#[allow(dead_code)]
pub struct Cartridge
{
    rom: Vec<u8>,
    ram: Vec<u8>
}

impl Cartridge
{
    pub fn read_rom(path: &str) -> Self
    {
        let rom: Vec<u8> = match read(Path::new(path))
        {
            Ok(d) => { 
                println!("Read ok. Bytes read: {}B | {}KiB", d.len(), (d.len() / 1024));
                d 
            },
            Err(err) => {
                println!("Failed to read ROM at \"{}\". The file may not exist or you do not have access to it. (Technical error: {})", path, err);
                todo!();
            },
        };

        Cartridge { 
            rom,
            ram: Vec::new()
        }
    }

    pub fn read_cart_data(&self) 
    {
        // Read off header bytes

        // Title
        let name: &str = &self.rom[0x134..=0x143] // Take the bytes that hold the title,
            .iter() // turn it into an iterator,
            .map(| n | *n as char) // map the bytes to chars,
            .collect::<String>(); // and collect it into a String

        println!("Cartridge title: {}", name);

        // Logo
        let logo: [u8; 48] = self.rom[0x104..=0x133].try_into().unwrap();
        if logo != LOGO_DUMP
        {
            println!("Invalid Nintendo logo. This may mean the ROM is corrupt, or it is an unofficial cartridge.");
        } else {
            println!("Valid Nintendo logo found.");
        }

        // ROM size
        println!("ROM size: {}KiB", 32 << self.rom[0x148]);

        // RAM size
        println!("RAM size: {}KiB", match self.rom[0x149] {
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
}