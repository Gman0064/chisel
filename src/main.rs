// main.rs
// Author: Garrett Dickinson
// Created: 01/23/2023
// Description: Main entrypoint script for chisel. Contains basic procedures
//              for gathering ELF file and program data.

use std::path;
use std::env;
use std::fs;
use std::process::exit;

// Import modules
mod elf;
mod util;


fn main() {
    // Collect our execution args
    let args: Vec<String> = env::args().collect();

    // Grab our filepath from our options
    if &args.len() < &2 {
        // No file given, terminate
        println!("[Error] Please provide a filepath to open");
        exit(0);
    }

    let file_path: &String = &args[1];
    
    if path::Path::new(file_path).exists() {
        println!("File exists, reading '{}'...", file_path);
        
        let contents: Result<Vec<u8>, std::io::Error> = fs::read(file_path);
        
        if contents.is_ok() {
            let bytes: &Vec<u8> = &contents.expect("");
            let magic_num: &[u8] = &bytes[0..4];
        
            if magic_num == elf::MAGIC_NUMBER {
                println!("Found ELF Magic Number...");
                println!("Parsing File Header...");

                // Build the File Header data structure
                let file_header: elf::FileHeader = build_file_header(bytes);

                // Build Program Header data structure
                //let program_header: elf::ProgramHeader = build_program_header(bytes, file_header.is_x86_64);
                
                println!("{:?}", file_header);
                //println!("{:?}", program_header);

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


fn build_file_header(data: &Vec<u8>) -> elf::FileHeader {
    
    // Determine x86 or x64 architecture
    // 0 : x86
    // 1 : x64
    let arch: usize = (data[elf::ARCH_OFFSET as usize] - 1).into();

    let file_header: elf::FileHeader = elf::FileHeader {
        arch: util::parse_architecture(data[elf::ARCH_OFFSET as usize]),
        is_x86_64: arch != 0,
        endian: util::parse_endian(data[elf::ENDIAN_OFFSET as usize]),
        abi: data[elf::ABI_OFFSET as usize],
        elf_type: data[elf::TYPE_OFFSET as usize],
        isa: data[elf::MACHINE_OFFSET as usize],
        entryoff: data[elf::ENTRYPOINT_OFFSET as usize],
        phoff: data[elf::PHOFF_OFFSET[arch] as usize],
        shoff: data[elf::SHOFF_OFFSET[arch] as usize],
        ehsize: data[elf::EHSIZE_OFFSET[arch] as usize],
        phentsize: data[elf::PHENTSIZE_OFFSET[arch] as usize],
        phnum: data[elf::PHNUM_OFFSET[arch] as usize],
        shentsize: data[elf::SHENTSIZE_OFFSET[arch] as usize],
        shnum: data[elf::SHNUM_OFFSET[arch] as usize],
        shstrndx: data[elf::SHSTRNDX_OFFSET[arch] as usize],
    };

    return file_header;
}


// fn build_program_header(data: &Vec<u8>, is_x86_64: bool) -> elf::ProgramHeader  {

//     let arch: i8 = if is_x86_64 { 1 } else { 0 };

//     let mut program_header: elf::ProgramHeader;

//     // let mut program_header: elf::ProgramHeader = elf::ProgramHeader {
//     //     arch: util::parse_architecture(data[elf::ARCH_OFFSET as usize])
//     // };

//     return program_header;
// }