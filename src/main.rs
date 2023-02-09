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


                // TODO: This is fundamentally wrong. Using the phentsize and phnum
                //       values from the file header, iterate over the program header
                //       table to find all of the individual program headers. There is
                //       not just one over-arching program header

                // Build Program Header data structure
                let program_header: elf::ProgramHeader = build_program_header(
                    bytes, 
                    file_header.phoff,
                    file_header.is_x86_64
                );


                // TODO: Same thing applies for the Section Headers...

                // Build Section Header data structure
                let section_header: elf::SectionHeader = build_section_header(
                    bytes, 
                    file_header.shoff,
                    file_header.is_x86_64
                );
                

                println!("{:?}", file_header);
                println!("{:?}", program_header);
                println!("{:?}", section_header);

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


fn build_program_header(data: &Vec<u8>, phoffset: u8, is_x86_64: bool) -> elf::ProgramHeader  {

    // Cast the supplied is_x86_64 bool to an array offset
    // 0 : x86
    // 1 : x64
    let arch: usize = is_x86_64.into();

    let program_header: elf::ProgramHeader = elf::ProgramHeader {
        program_type: data[(elf::PH_TYPE_OFFSET + phoffset) as usize],
        flags: data[(elf::PH_FLAGS_OFFSET[arch] + phoffset) as usize],
        offset: data[(elf::PH_OFFSET_OFFSET[arch] + phoffset) as usize],
        vaddr: data[(elf::PH_VADDR_OFFSET[arch] + phoffset) as usize],
        paddr: data[(elf::PH_PADDR_OFFSET[arch] + phoffset) as usize],
        filesz: data[(elf::PH_FILESZ_OFFSET[arch] + phoffset) as usize],
        memsz: data[(elf::PH_MEMSZ_OFFSET[arch] + phoffset) as usize],
        align: data[(elf::PH_ALIGN_OFFSET[arch] + phoffset) as usize],
    };

    return program_header;
}


fn build_section_header(data: &Vec<u8>, shoffset: u8, is_x86_64: bool) -> elf::SectionHeader  {

    // Cast the supplied is_x86_64 bool to an array offset
    // 0 : x86
    // 1 : x64
    let arch: usize = is_x86_64.into();

    let section_header: elf::SectionHeader = elf::SectionHeader {
        name: data[(elf::SH_NAME_OFFSET + shoffset) as usize],
        section_type: data[(elf::SH_TYPE_OFFSET + shoffset) as usize],
        flags: data[(elf::SH_FLAGS_OFFSET + shoffset) as usize],
        addr: data[(elf::SH_ADDR_OFFSET[arch] + shoffset) as usize],
        offset: data[(elf::SH_OFFSET_OFFSET[arch] + shoffset) as usize],
        size: data[(elf::SH_SIZE_OFFSET[arch] + shoffset) as usize],
        link: data[(elf::SH_LINK_OFFSET[arch] + shoffset) as usize],
        info: data[(elf::SH_INFO_OFFSET[arch] + shoffset) as usize],
        addralign: data[(elf::SH_ADDRALIGN_OFFSET[arch] + shoffset) as usize],
        entsize: data[(elf::SH_ENTSIZE_OFFSET[arch] + shoffset) as usize],
    };

    return section_header;
}