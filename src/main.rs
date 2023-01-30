// main.rs
// Author: Garrett Dickinson
// Created: 01/23/2023
// Description: Main entrypoint script for chisel. Contains basic procedures
//              for gathering ELF file and program data.

use std::path;
use std::env;
use std::fs;
use std::process::exit;
use std::collections::HashMap;


// ELF Header Sizes

const ELF_FILE_HEADER_LENGTH: [u8; 2] = [0x34, 0x40];


// Generic ELF information offsets.

const ELF_MAGIC_NUMBER: &[u8] = &[0x7F,0x45,0x4C,0x46];
const ELF_ARCH_OFFSET: u8 = 0x04;       // x86 or x64 indiicator; 1 byte
const ELF_ENDIAN_OFFSET: u8 = 0x05;     // Endian offset (1 - little, 2 - big); 1 byte
const ELF_ABI_OFFSET: u8 = 0x07;        // ABI identifier; 1 byte
const ELF_TYPE_OFFSET: u8 = 0x10;       // Object type identifier; 2 bytes
const ELF_MACHINE_OFFSET: u8 = 0x12;    // Instruction set type; 2 bytes


// Offsets for file header entry points and table inforamtion.
// Arrayed offset are split by architecture:
//      0 : x86
//      1 : x86_64

const ELF_ENTRYPOINT_OFFSET: u8 = 0x18;
const ELF_PHOFF_OFFSET: [u8; 2] = [0x1C, 0x20];        // Program header table pointer; 2 bytes
const ELF_SHOFF_OFFSET: [u8; 2] = [0x20, 0x28];        // Section table pointer; 2 bytes
const ELF_EHSIZE_OFFSET: [u8; 2] = [0x28, 0x34];       // Program header table entry size pointer; 2 bytes
const ELF_PHENTSIZE_OFFSET: [u8; 2] = [0x28, 0x34];    // Section table pointer; 2 bytes
const ELF_PHNUM_OFFSET: [u8; 2] = [0x2C, 0x38];        // Program header table number of entries pointer; 2 bytes
const ELF_SHENTSIZE_OFFSET: [u8; 2] = [0x2E, 0x3A];    // Size of section header table; 2 bytes
const ELF_SHNUM_OFFSET: [u8; 2] = [0x30, 0x3C];        // Number of entries in section table pointer; 2 bytes
const ELF_SHSTRNDX_OFFSET: [u8; 2] = [0x32, 0x3E];     // Index of section header that contains names; 2 bytes


fn main() {
    // Collect our execution args
    let args: Vec<String> = env::args().collect();

    // Grab our filepath from our options
    if &args.len() < &2 {
        // No file given, terminate
        println!("[Error] Please provied a file to open...");
        exit(0);
    }

    let file_path: &String = &args[1];
    
    if path::Path::new(file_path).exists() {
        println!("File exists, reading '{}'", file_path);
        
        let contents: Result<Vec<u8>, std::io::Error> = fs::read(file_path);
        
        if contents.is_ok() {
            let bytes: &Vec<u8> = &contents.expect("");
            let magic_num: &[u8] = &bytes[0..4];
        
            if magic_num == ELF_MAGIC_NUMBER {
                println!("Found ELF Magic Number...");
                println!("Parsing File Header...");

                // Build the File Header data structure
                let file_header_map = build_fild_header(bytes);

                for (key, value) in &file_header_map {
                    println!("{}: {}", key, value);
                }

            } else {
                println!("[Error] Could not find magic number, is this an ELF executable?")
            }
        }
    } else {
        println!("[Error] '{}' does not exist", file_path);
        exit(-1);
    }

    return;
}


fn build_fild_header(data: &Vec<u8>) -> HashMap<String, u8>{
    let mut file_header: HashMap<String, u8> = HashMap::new();
    
    // Determine x86 or x64 architecture
    // 0 : x86
    // 1 : x64
    let arch: u8 = (data[ELF_ARCH_OFFSET as usize] - 1).into();

    file_header.insert("e_arch".to_string(), data[ELF_ARCH_OFFSET as usize]);
    file_header.insert("e_endian".to_string(), data[ELF_ENDIAN_OFFSET as usize]);
    file_header.insert("e_abi".to_string(), data[ELF_ABI_OFFSET as usize]);
    file_header.insert("e_type".to_string(), data[ELF_TYPE_OFFSET as usize]);
    file_header.insert("e_machine".to_string(), data[ELF_MACHINE_OFFSET as usize]);

    file_header.insert("e_entry".to_string(), data[ELF_ENTRYPOINT_OFFSET as usize]);
    file_header.insert("e_phoff".to_string(), data[ELF_PHOFF_OFFSET[arch as usize] as usize]);
    file_header.insert("e_shoff".to_string(), data[ELF_SHOFF_OFFSET[arch as usize] as usize]);
    file_header.insert("e_ehsize".to_string(), data[ELF_EHSIZE_OFFSET[arch as usize] as usize]);
    file_header.insert("e_phentsize".to_string(), data[ELF_PHENTSIZE_OFFSET[arch as usize] as usize]);
    file_header.insert("e_phnum".to_string(), data[ELF_PHNUM_OFFSET[arch as usize] as usize]);
    file_header.insert("e_shentsize".to_string(), data[ELF_SHENTSIZE_OFFSET[arch as usize] as usize]);
    file_header.insert("e_shnum".to_string(), data[ELF_SHNUM_OFFSET[arch as usize] as usize]);
    file_header.insert("e_shstrndx".to_string(), data[ELF_SHSTRNDX_OFFSET[arch as usize] as usize]);

    return file_header;
}