use std::{fs::read, path::Path};

pub struct Cartridge
{
    memory: [u8; 65536],
}

pub fn read_rom(path: &str) -> Cartridge
{
    let data: [u8; 65536] = match read(Path::new(path))
    {
        Ok(d) => { d },
        Err(_err) => {
            todo!(); // TODO Decide what to do if file doesn't exist
        }
    }.try_into().unwrap_or_else(| n: Vec<u8> | {
        if n.len() < 65536 
        {
            while n.len() < 65536
            {
                n.push(0);
            }

            return n.try_into().unwrap();
        }
    });

    Cartridge {
        memory: data
    }
}