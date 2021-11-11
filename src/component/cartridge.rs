#![allow(dead_code)]

use std::{fs::read, path::Path};

use crate::lookups;

pub struct Cartridge
{
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    
    meta: CartridgeMeta
}

pub struct CartridgeMeta
{
    title: String,
    sgb_supported: bool,
    cart_specs: (String, u16, u8, bool, bool),
    rom_size: u16,
    destination: String,
    version: u8
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

        let meta: CartridgeMeta = {
            populate_cart_meta(&rom)
        };

        Cartridge { 
            rom,
            ram: Vec::new(),
            meta
        }
    }

    
}

fn populate_cart_meta(data: &Vec<u8>) -> CartridgeMeta
{
    let title: String = data[0x134..=0x143]
        .iter()
        .map(| b | *b as char)
        .collect::<String>();

    let sgb_supported = data[0x146] == 0x03;

    let pre_specs = lookups::mbc_type(data[0x147]);
    let cart_specs = (pre_specs.0.to_string(), pre_specs.1, pre_specs.2, pre_specs.3, pre_specs.4);

    let rom_size: u16 = 32 << data[0x148];

    let destination: String = match data[0x14A]
    {
        0x00 => "Japan".to_string(),
        _ => "Global".to_string()
    };

    let version: u8 = data[0x14C];

    CartridgeMeta {
        title,
        sgb_supported,
        cart_specs,
        rom_size,
        destination,
        version,
    }
}