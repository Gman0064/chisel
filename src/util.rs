// util.rs
// Author: Garrett Dickinson
// Created: 02/02/2023
// Description: Utility script for storing common-use and helper
//              functions.

use std::mem;

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


pub fn parse_isa(isa: u16) -> String {
    match isa {
        0x03 => "Intel x86".to_string(),
        0x3E => "AMD x86-64".to_string(),

        // Matching just for fun, maybe future functionality? o.O
        0x14 => "PowerPC".to_string(),
        0x15 => "PowerPC 64-bit".to_string(),
        0x32 => "IA_64".to_string(),
        0x28 => "Arm".to_string(),
        0xB7 => "Arm 64-bit".to_string(),
        0xF3 => "RISC-V".to_string(),

        // Match unknown ISA
        _ => "Unknown".to_string()
    }
}


pub fn u16_from_buffer(buff: &Vec<u8>, index: usize) -> u16 {
    const SIZE: usize = mem::size_of::<u16>();

    let mut slice: [u8; SIZE] = [0; SIZE];
    slice.copy_from_slice(&buff[index..index+SIZE]);

    let value: u16 = u16::from_ne_bytes(slice);

    return value;
}


pub fn u32_from_buffer(buff: &Vec<u8>, index: usize) -> u32 {
    const SIZE: usize = mem::size_of::<u32>();

    let mut slice: [u8; SIZE] = [0; SIZE];
    slice.copy_from_slice(&buff[index..index+SIZE]);

    let value: u32 = u32::from_ne_bytes(slice);

    return value;
}


pub fn u64_from_buffer(buff: &Vec<u8>, index: usize) -> u64 {
    const SIZE: usize = mem::size_of::<u64>();

    let mut slice: [u8; SIZE] = [0; SIZE];
    slice.copy_from_slice(&buff[index..index+SIZE]);

    let value: u64 = u64::from_ne_bytes(slice);

    return value;
}


pub fn parse_section_name(buff: &Vec<u8>, index: usize) -> String {
    let mut name: Vec<u8> = Vec::new();
    let mut char_ctr: usize = index;
    let mut char: u8 = buff[index];
    
    while char != 0x00 {
        name.push(char);
        char_ctr += 1;
        char = buff[char_ctr];
    }

    let result = String::from_utf8(name).expect("Failed to parse section name!");
    
    return result;
}


pub fn parse_program_segment_type(segment_type: u32) -> String {
    match segment_type {
        0x00000000 => "PT_NULL".to_string(),
        0x00000001 => "PT_LOAD".to_string(),
        0x00000002 => "PT_DYNAMIC".to_string(),
        0x00000003 => "PT_INTERP".to_string(),
        0x00000004 => "PT_NOTE".to_string(),
        0x00000005 => "PT_SHLIB".to_string(),
        0x00000006 => "PT_PHDR".to_string(),
        0x00000007 => "PT_TLS".to_string(),
        0x60000000 => "PT_LOOS".to_string(),
        0x6FFFFFFF => "PT_HIOS".to_string(),
        0x70000000 => "PT_LOPROC".to_string(),
        0x7FFFFFFF => "PT_HIPROC".to_string(),

        // Match unknown segment type
        _ => "UNKNOWN".to_string()
    }
}