// util.rs
// Author: Garrett Dickinson
// Created: 02/02/2023
// Description: Utility script for storing common-use and helper
//              functions.

use crate::elf::{self, EndianType, ArchitectureType};


pub fn parse_endian(endian: u8) -> elf::EndianType {
    match endian {
        0x00 => return EndianType::Big,
        0x01 => return EndianType::Little,
        _ => return EndianType::Unknown
    }
}


pub fn parse_architecture(arch: u8) -> elf::ArchitectureType {
    match arch {
        0x01 => return ArchitectureType::X86,
        0x02 => return ArchitectureType::X86_64,
        _ => return ArchitectureType::Unknown
    }
}


pub fn parse_abi(abi: u8) -> String {
    match abi {
        0x00 => "SystemV".to_string(),
        0x01 => "HP-UX".to_string(),
        0x02 => "NetBSD".to_string(),
        0x03 => "Linux".to_string(),
        0x04 => "GNU Hurd".to_string(),
        0x06 => "Solaris".to_string(),
        0x07 => "AIX".to_string(),
        0x08 => "IRIX".to_string(),
        0x09 => "FreeBSD".to_string(),
        0x0C => "OpenBSD".to_string(),
        0x0D => "OpenVMS".to_string(),

        // Match unknown ABI
        _ => "Unknown".to_string()
    }
}


pub fn parse_isa(isa: u8) -> String {
    match isa {
        0x03 => "Intel x86".to_string(),
        0x3E => "AMD x86-64".to_string(),

        // Matching just for fun, maybe future functionality? o.O
        0x14 => "PowerPC".to_string(),
        0x15 => "PowerPC 64-bit".to_string(),
        0x32 => "IA_64".to_string(),
        0x28 => "Arm".to_string(),
        0xB7 => "Arm 64-bit".to_string(),

        // Match unknown ISA
        _ => "Unknown".to_string()
    }
}

