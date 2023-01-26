// main.rs
// Author: Garrett Dickinson
// Created: 01/23/2023
// Description: Main entrypoint script for chisel. Contains basic procedures
//              for gathering ELF file data.


use std::path;
use std::env;
use std::fs;
use std::process::exit;

// Generic ELF information offsets.

const ELF_MAGIC_NUMBER: &[u8] = &[0x7F,0x45,0x4C,0x46];
const ELF_ARCH_OFFSET: u8 = 0x04;       // x86 or x64 indiicator; 1 byte
const ELF_ENDIAN_OFFSET: u8 = 0x05;     // Endian offset (1 - little, 2 - big); 1 byte
const ELF_ABI_OFFSET: u8 = 0x07;        // ABI identifier; 1 byte
const ELF_TYPE_OFFSET: u8 = 0x10;       // Object type identifier; 2 bytes
const ELF_MACHINE_OFFSET: u8 = 0x12;    // Instruction set type; 2 bytes


// Entry points and program header table inforamtion.
// Tupled offset are split by architecture:
//      0 : x86
//      1 : x86_64

const ELF_ENTRYPOINT_OFFSET: u8 = 0x18;
const ELF_PHOFF_OFFSET: (u8, u8) = (0x1C, 0x20);        // Program header table pointer; 2 bytes
const ELF_SHOFF_OFFSET: (u8, u8) = (0x20, 0x28);        // Section table pointer; 2 bytes
const ELF_EHSIZE_OFFSET: (u8, u8) = (0x28, 0x34);       // Program header table entry size pointer; 2 bytes
const ELF_PHENTSIZE_OFFSET: (u8, u8) = (0x28, 0x34);    // Section table pointer; 2 bytes
const ELF_PHNUM_OFFSET: (u8, u8) = (0x2C, 0x38);
const ELF_SHENTSIZE_OFFSET: (u8, u8) = (0x2E, 0x3A);
const ELF_SHNUM_OFFSET: (u8, u8) = (0x30, 0x3C);
const ELF_SHSTRNDX_OFFSET: (u8, u8) = (0x32, 0x3E);


fn main() {
    // Collect our execution args
    let args: Vec<String> = env::args().collect();

    // Grab our filepath from our options
    let file_path = &args[1];
    
    if path::Path::new(file_path).exists() {
        println!("File exists, reading '{}'", file_path);
        
        let contents: Result<Vec<u8>, std::io::Error> = fs::read(file_path);
        
        if contents.is_ok() {
            let bytes: &Vec<u8> = &contents.expect("");
            let magic_num: &[u8] = &bytes[0..4];
        
            if magic_num == ELF_MAGIC_NUMBER {
                println!("Found ELF Magic Number!");
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
