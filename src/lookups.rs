#![allow(dead_code)]

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
        0x06 | 0x0E | 0x16 | 0x18 | 0x1E | 0x20 | 0x26 | 0x28 | 0x2E | 0x30 | 0x36 | 0x38 | 0x3E | 0xC6 | 0xCB | 0xCE | 0xD6 | 0xDE | 0xE0 | 0xE6 | 0xE8 | 0xEE | 0xF0 | 0xF6 | 0xF8 | 0xFE => {
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

pub fn exit_codes(exit_code: u8) -> &'static str
{
    match exit_code
    {
        0 => "Ok",
        1 => "General error",
        3 => "Unimplemented opcode",
        4 => "Unknown opcode",
        6 => "Unknown prefixed opcode",
        _ => "Undefined"
    }
}

/// Opcode master list
pub const GRAND_OPCODE: [(u8, &str, u8); 256] = [
    (0x00, "NOP", 1), 
    (0x01, "LD", 3),  
    (0x02, "LD", 1),  
    (0x03, "INC", 1), 
    (0x04, "INC", 1), 
    (0x05, "DEC", 1), 
    (0x06, "LD", 2),  
    (0x07, "RLCA", 1),
    (0x08, "LD", 3),  
    (0x09, "ADD", 1), 
    (0x0A, "LD", 1),  
    (0x0B, "DEC", 1), 
    (0x0C, "INC", 1), 
    (0x0D, "DEC", 1), 
    (0x0E, "LD", 2),
    (0x0F, "RRCA", 1),
    (0x10, "STOP", 2),
    (0x11, "LD", 3),
    (0x12, "LD", 1),
    (0x13, "INC", 1),
    (0x14, "INC", 1),
    (0x15, "DEC", 1),
    (0x16, "LD", 2),
    (0x17, "RLA", 1),
    (0x18, "JR", 2),
    (0x19, "ADD", 1),
    (0x1A, "LD", 1),
    (0x1B, "DEC", 1),
    (0x1C, "INC", 1),
    (0x1D, "DEC", 1),
    (0x1E, "LD", 2),
    (0x1F, "RRA", 1),
    (0x20, "JR_NZ", 2),
    (0x21, "LD", 3),
    (0x22, "LD", 1),
    (0x23, "INC", 1),
    (0x24, "INC", 1),
    (0x25, "DEC", 1),
    (0x26, "LD", 2),
    (0x27, "DAA", 1),
    (0x28, "JR_Z", 2),
    (0x29, "ADD", 1),
    (0x2A, "LD", 1),
    (0x2B, "DEC", 1),
    (0x2C, "INC", 1),
    (0x2D, "DEC", 1),
    (0x2E, "LD", 2),
    (0x2F, "CPL", 1),
    (0x30, "JR_NC", 2),
    (0x31, "LD", 3),
    (0x32, "LD", 1),
    (0x33, "INC", 1),
    (0x34, "INC", 1),
    (0x35, "DEC", 1),
    (0x36, "LD", 2),
    (0x37, "SCF", 1),
    (0x38, "JR_C", 2),
    (0x39, "ADD", 1),
    (0x3A, "LD", 1),
    (0x3B, "DEC", 1),
    (0x3C, "INC", 1),
    (0x3D, "DEC", 1),
    (0x3E, "LD_A_8", 2),
    (0x3F, "CCF", 1),
    (0x40, "LD", 1),
    (0x41, "LD", 1),
    (0x42, "LD", 1),
    (0x43, "LD", 1),
    (0x44, "LD", 1),
    (0x45, "LD", 1),
    (0x46, "LD", 1),
    (0x47, "LD", 1),
    (0x48, "LD", 1),
    (0x49, "LD", 1),
    (0x4A, "LD", 1),
    (0x4B, "LD", 1),
    (0x4C, "LD", 1),
    (0x4D, "LD", 1),
    (0x4E, "LD", 1),
    (0x4F, "LD", 1),
    (0x50, "LD", 1),
    (0x51, "LD", 1),
    (0x52, "LD", 1),
    (0x53, "LD", 1),
    (0x54, "LD", 1),
    (0x55, "LD", 1),
    (0x56, "LD", 1),
    (0x57, "LD", 1),
    (0x58, "LD", 1),
    (0x59, "LD", 1),
    (0x5A, "LD", 1),
    (0x5B, "LD", 1),
    (0x5C, "LD", 1),
    (0x5D, "LD", 1),
    (0x5E, "LD", 1),
    (0x5F, "LD", 1),
    (0x60, "LD", 1),
    (0x61, "LD", 1),
    (0x62, "LD", 1),
    (0x63, "LD", 1),
    (0x64, "LD", 1),
    (0x65, "LD", 1),
    (0x66, "LD", 1),
    (0x67, "LD", 1),
    (0x68, "LD", 1),
    (0x69, "LD", 1),
    (0x6A, "LD", 1),
    (0x6B, "LD", 1),
    (0x6C, "LD", 1),
    (0x6D, "LD", 1),
    (0x6E, "LD", 1),
    (0x6F, "LD", 1),
    (0x70, "LD", 1),
    (0x71, "LD", 1),
    (0x72, "LD", 1),
    (0x73, "LD", 1),
    (0x74, "LD", 1),
    (0x75, "LD", 1),
    (0x76, "HALT", 1),
    (0x77, "LD", 1),
    (0x78, "LD", 1),
    (0x79, "LD", 1),
    (0x7A, "LD", 1),
    (0x7B, "LD", 1),
    (0x7C, "LD", 1),
    (0x7D, "LD", 1),
    (0x7E, "LD", 1),
    (0x7F, "LD", 1),
    (0x80, "ADD", 1),
    (0x81, "ADD", 1),
    (0x82, "ADD", 1),
    (0x83, "ADD", 1),
    (0x84, "ADD", 1),
    (0x85, "ADD", 1),
    (0x86, "ADD", 1),
    (0x87, "ADD", 1),
    (0x88, "ADC", 1),
    (0x89, "ADC", 1),
    (0x8A, "ADC", 1),
    (0x8B, "ADC", 1),
    (0x8C, "ADC", 1),
    (0x8D, "ADC", 1),
    (0x8E, "ADC", 1),
    (0x8F, "ADC", 1),
    (0x90, "SUB", 1),
    (0x91, "SUB", 1),
    (0x92, "SUB", 1),
    (0x93, "SUB", 1),
    (0x94, "SUB", 1),
    (0x95, "SUB", 1),
    (0x96, "SUB", 1),
    (0x97, "SUB", 1),
    (0x98, "SBC", 1),
    (0x99, "SBC", 1),
    (0x9A, "SBC", 1),
    (0x9B, "SBC", 1),
    (0x9C, "SBC", 1),
    (0x9D, "SBC", 1),
    (0x9E, "SBC", 1),
    (0x9F, "SBC", 1),
    (0xA0, "AND", 1),
    (0xA1, "AND", 1),
    (0xA2, "AND", 1),
    (0xA3, "AND", 1),
    (0xA4, "AND", 1),
    (0xA5, "AND", 1),
    (0xA6, "AND", 1),
    (0xA7, "AND", 1),
    (0xA8, "XOR", 1),
    (0xA9, "XOR", 1),
    (0xAA, "XOR", 1),
    (0xAB, "XOR", 1),
    (0xAC, "XOR", 1),
    (0xAD, "XOR", 1),
    (0xAE, "XOR", 1),
    (0xAF, "XOR_A", 1),
    (0xB0, "OR", 1),
    (0xB1, "OR", 1),
    (0xB2, "OR", 1),
    (0xB3, "OR", 1),
    (0xB4, "OR", 1),
    (0xB5, "OR", 1),
    (0xB6, "OR", 1),
    (0xB7, "OR", 1),
    (0xB8, "CP", 1),
    (0xB9, "CP", 1),
    (0xBA, "CP", 1),
    (0xBB, "CP", 1),
    (0xBC, "CP", 1),
    (0xBD, "CP", 1),
    (0xBE, "CP", 1),
    (0xBF, "CP", 1),
    (0xC0, "RET", 1),
    (0xC1, "POP", 1),
    (0xC2, "JP", 3),
    (0xC3, "JP", 3),
    (0xC4, "CALL", 3),
    (0xC5, "PUSH", 1),
    (0xC6, "ADD", 2),
    (0xC7, "RST", 1),
    (0xC8, "RET", 1),
    (0xC9, "RET", 1),
    (0xCA, "JP", 3),
    (0xCB, "PRFIX", 1),
    (0xCC, "CALL", 3),
    (0xCD, "CALL", 3),
    (0xCE, "ADC", 2),
    (0xCF, "RST", 1),
    (0xD0, "RET", 1),
    (0xD1, "POP", 1),
    (0xD2, "JP", 3),
    (0xD3, "ILLEGAL_D3", 1),
    (0xD4, "CALL", 3),
    (0xD5, "PUSH", 1),
    (0xD6, "SUB", 2),
    (0xD7, "RST", 1),
    (0xD8, "RET", 1),
    (0xD9, "RETI", 1),
    (0xDA, "JP", 3),
    (0xDB, "ILLEGAL_DB", 1),
    (0xDC, "CALL", 3),
    (0xDD, "ILLEGAL_DD", 1),
    (0xDE, "SBC", 2),
    (0xDF, "RST", 1),
    (0xE0, "LD_a_A", 2),
    (0xE1, "POP", 1),
    (0xE2, "LD", 1),
    (0xE3, "ILLEGAL_E3", 1),
    (0xE4, "ILLEGAL_E4", 1),
    (0xE5, "PUSH", 1),
    (0xE6, "AND", 2),
    (0xE7, "RST", 1),
    (0xE8, "ADD", 2),
    (0xE9, "JP", 1),
    (0xEA, "LD_A", 3),
    (0xEB, "ILLEGAL_EB", 1),
    (0xEC, "ILLEGAL_EC", 1),
    (0xED, "ILLEGAL_ED", 1),
    (0xEE, "XOR", 2),
    (0xEF, "RST", 1),
    (0xF0, "LD_A,a", 2),
    (0xF1, "POP", 1),
    (0xF2, "LD", 1),
    (0xF3, "DI", 1),
    (0xF4, "ILLEGAL_F4", 1),
    (0xF5, "PUSH", 1),
    (0xF6, "OR", 2),
    (0xF7, "RST", 1),
    (0xF8, "LD", 2),
    (0xF9, "LD", 1),
    (0xFA, "LD", 3),
    (0xFB, "EI", 1),
    (0xFC, "ILLEGAL_FC", 1),
    (0xFD, "ILLEGAL_FD", 1),
    (0xFE, "CP", 2),
    (0xFF, "RST", 1),
];

pub fn grand_opcode_lookup(opcode: u8) -> (u8, &'static str, u8)
{
    GRAND_OPCODE[opcode as usize]
}