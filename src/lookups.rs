
/// Obtains a tuple containing: MBC type, number of ROM banks present, number of RAM banks present, and whether or not the cartride contains a battery
pub fn mbc_type(byte: u8) -> (&'static str, u16, u8, bool, bool)
{
    match byte
    {
        0x00 => {
            (
                "ROM ONLY",
                1,
                0,
                false,
                false
            )
        },
        0x01 => {
            (
                "MBC1",
                128,
                0,
                false,
                false
            )
        },
        0x02 => {
            (
                "MBC1 + RAM",
                128,
                4,
                false,
                false
            )
        },
        0x03 => {
            (
                "MBC1 + RAM + Battery",
                128,
                4,
                true,
                false
            )
        },
        0x05 => {
            (
                "MBC2",
                16,
                255,
                false,
                false
            )
        },
        0x06 => {
            (
                "MBC2 + Battery",
                16,
                255,
                false,
                false
            )
        },
        0x0F => {
            (
                "MBC3 + Battery + Timer",
                128,
                0,
                true,
                true
            )
        },
        0x10 => {
            (
                "MBC3 + RAM + Battery + Timer",
                128,
                4,
                true,
                true
            )
        },
        0x11 => {
            (
                "MBC3",
                128,
                0,
                false,
                false
            )
        },
        0x12 => {
            (
                "MBC3 + RAM",
                128,
                4,
                false,
                false
            )
        },
        0x13 => {
            (
                "MBC3 + RAM + Battery",
                128,
                4,
                true,
                false
            )
        },
        0x19 => {
            (
                "MBC5",
                512,
                16,
                false,
                false
            )
        }
        0x1A => {
            (
                "MBC5 + RAM",
                512,
                0,
                false,
                false
            )
        }
        0x1B => {
            (
                "MBC5 + RAM + Battery",
                512,
                16,
                true,
                false
            )
        }
        _ => {
            (
                "INVALID",
                0,
                0,
                false,
                false
            )
        }
    }
}

pub fn instruction_len(opcode: &u8) -> u8
{
    match *opcode
    {
        0x06 | 0x0E | 0x16 | 0x18 | 0x1E | 0x20 | 0x26 | 0x28 | 0x2E | 0x30 | 0x36 | 0x38 | 0x3E | 0xC6 | 0xCE | 0xD6 | 0xDE | 0xE0 | 0xE6 | 0xE8 | 0xEE | 0xF0 | 0xF6 | 0xF8 | 0xFE => {
            2
        },
        0x01 | 0x08 | 0x11 | 0x21 | 0x31 | 0xC2 | 0xC3 | 0xC4 | 0xCA | 0xCC | 0xCD | 0xD2 | 0xD4 | 0xDA | 0xDC | 0xEA | 0xFA => {
            3
        },
        _ => {
            1
        }
    }
}