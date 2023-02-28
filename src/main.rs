// main.rs
// Author: Garrett Dickinson
// Created: 01/23/2023
// Description: Main entrypoint script for chisel. Contains basic procedures
//              for gathering ELF file and program data.

use iced_x86::*;
use std::collections::HashMap;
use std::io::Write;
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

                println!("\t- Found {} program header entries {} bytes in length", file_header.phnum, file_header.phentsize);
                println!("\t- Found {} section header entries {} bytes in length", file_header.shnum, file_header.shentsize);
                println!("\t- Found .shstrtab section at index {}", file_header.shstrndx); 

                println!("{:?}", file_header);

                println!("\nParsing Section Headers...");


                // Determine the shstrtab offset.
                // This is found by taking the string table index and multiplying it by the section header entry size, then
                // adding this to the initial section header offset.
                let shstrtab_offset: u64 = file_header.shoff + (file_header.shentsize as u64 * file_header.shstrndx as u64);

                // Build a read-only version of the .shstrtab section
                let shstrtab_section: elf::SectionHeader = build_section_header(
                    bytes,
                    shstrtab_offset as usize,
                    file_header.is_x86_64
                );

                let shstrtab_start: u64 = shstrtab_section.offset;
                let shstrtab_end: u64 = shstrtab_section.offset + shstrtab_section.size;
                let shstrtab_data: Vec<u8> = bytes[shstrtab_start as usize..shstrtab_end as usize].to_vec();

                println!("\t- Found .shstrtab section");


                println!("\n=== Sections ===");

                let mut section_table_map: HashMap<String, elf::SectionHeader> = HashMap::new();
                let mut section_table_offset: u64 = file_header.shoff;
                let mut section_table_count: i32 = 0;

                // Build Section Header data structure
                for _ in 0..file_header.shnum {
                    let section_header: elf::SectionHeader = build_section_header(
                        bytes, 
                        section_table_offset as usize,
                        file_header.is_x86_64
                    );

                    // Determine the section name for each section using the shstrtab data
                    let section_name: String = util::parse_section_name(&shstrtab_data, section_header.name as usize);

                    println!("[{}] {}", section_table_count, section_name);
                    println!("{:?}", section_header);

                    section_table_map.insert(section_name, section_header);

                    section_table_offset += file_header.shentsize as u64;
                    section_table_count += 1;
                }

                println!("\nParsing Program Segments...");
                
                println!("\n=== Program Segments ===");

                let mut program_table_offset = file_header.phoff;
                let mut program_table_count: i32 = 0;

                // Build Section Header data structure
                for _ in 0..file_header.phnum {
                    // Build Program Header data structure
                    let program_header: elf::ProgramHeader = build_program_header(
                        bytes, 
                        program_table_offset as usize,
                        file_header.is_x86_64
                    );

                    // Set a default section name if there's no index found in the table
                    let program_name: String = util::parse_program_segment_type(program_header.program_type);

                    println!("[{}] {}", program_table_count, program_name);
                    println!("{:?}", program_header);

                    program_table_offset += file_header.phentsize as u64;
                    program_table_count += 1;
                }


                // Now that we have all the sections, spit out the .text section and start a linear disassembly
                let text_section: &elf::SectionHeader = section_table_map.get(".text").unwrap();
                let text_section_offset: usize = text_section.offset as usize;
                let text_section_end: usize = text_section_offset + text_section.size as usize;

                let text_section_buff: &[u8] = &bytes[text_section_offset..text_section_end];
                
                let mut decoder: Decoder = Decoder::new(64, text_section_buff, DecoderOptions::NONE);
                let mut instruction: Instruction = Instruction::default();
                let instruction_start: u64 = 0;
                let instruction_length: u64 = 1;

                while (instruction_start + instruction_length) < text_section.size {
                    //let instruction_bytes: &[u8] = &text_section_buff[instruction_start..instruction_length];
                    //instruction.
                    //decoder.decode_out(instruction)
                }

                //let out_file = fs::File::create("out.s").unwrap();
                //out_file.write()
                //out_file.flush();

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
        abi_str: util::parse_abi(data[elf::ABI_OFFSET as usize]),
        elf_type: util::u16_from_buffer(data, elf::TYPE_OFFSET as usize),
        isa: util::u16_from_buffer(data, elf::MACHINE_OFFSET as usize),
        isa_str: util::parse_isa(util::u16_from_buffer(data, elf::MACHINE_OFFSET as usize)),
        entryoff: util::u64_from_buffer(data, elf::ENTRYPOINT_OFFSET as usize),
        phoff: util::u64_from_buffer(data, elf::PHOFF_OFFSET[arch] as usize),
        shoff: util::u64_from_buffer(data, elf::SHOFF_OFFSET[arch] as usize),
        ehsize: util::u16_from_buffer(data, elf::EHSIZE_OFFSET[arch] as usize),
        phentsize: util::u16_from_buffer(data, elf::PHENTSIZE_OFFSET[arch] as usize),
        phnum: util::u16_from_buffer(data, elf::PHNUM_OFFSET[arch] as usize),
        shentsize: util::u16_from_buffer(data, elf::SHENTSIZE_OFFSET[arch] as usize),
        shnum: util::u16_from_buffer(data, elf::SHNUM_OFFSET[arch] as usize),
        shstrndx: util::u16_from_buffer(data, elf::SHSTRNDX_OFFSET[arch] as usize),
    };

    return file_header;
}


fn build_program_header(data: &Vec<u8>, phoffset: usize, is_x86_64: bool) -> elf::ProgramHeader  {

    // Cast the supplied is_x86_64 bool to an array offset
    // 0 : x86
    // 1 : x64
    let arch: usize = is_x86_64.into();

    let program_header: elf::ProgramHeader = elf::ProgramHeader {
        program_type: util::u32_from_buffer(data, phoffset + elf::PH_TYPE_OFFSET as usize),
        flags: util::u32_from_buffer(data, phoffset + elf::PH_FLAGS_OFFSET[arch] as usize),
        offset: util::u64_from_buffer(data, phoffset + elf::PH_OFFSET_OFFSET[arch] as usize),
        vaddr: util::u64_from_buffer(data, phoffset + elf::PH_VADDR_OFFSET[arch] as usize),
        paddr: util::u64_from_buffer(data, phoffset + elf::PH_PADDR_OFFSET[arch] as usize),
        filesz: util::u64_from_buffer(data, phoffset + elf::PH_FILESZ_OFFSET[arch] as usize),
        memsz: util::u64_from_buffer(data, phoffset + elf::PH_MEMSZ_OFFSET[arch] as usize),
        align: util::u64_from_buffer(data, phoffset + elf::PH_ALIGN_OFFSET[arch] as usize)
    };

    return program_header;
}


fn build_section_header(data: &Vec<u8>, shoffset: usize, is_x86_64: bool) -> elf::SectionHeader  {

    // Cast the supplied is_x86_64 bool to an array offset
    // 0 : x86
    // 1 : x64
    let arch: usize = is_x86_64.into();

    let section_header: elf::SectionHeader = elf::SectionHeader {
        name: util::u32_from_buffer(data, shoffset + elf::SH_NAME_OFFSET as usize),
        section_type: util::u32_from_buffer(data, shoffset + elf::SH_TYPE_OFFSET as usize),
        flags: util::u64_from_buffer(data, shoffset + elf::SH_FLAGS_OFFSET as usize),
        addr: util::u64_from_buffer(data, shoffset + elf::SH_ADDR_OFFSET[arch] as usize),
        offset: util::u64_from_buffer(data, shoffset + elf::SH_OFFSET_OFFSET[arch] as usize),
        size: util::u64_from_buffer(data, shoffset + elf::SH_SIZE_OFFSET[arch] as usize),
        link: util::u32_from_buffer(data, shoffset + elf::SH_LINK_OFFSET[arch] as usize),
        info: util::u32_from_buffer(data, shoffset + elf::SH_INFO_OFFSET[arch] as usize),
        addralign: util::u64_from_buffer(data, shoffset + elf::SH_ADDRALIGN_OFFSET[arch] as usize),
        entsize: util::u64_from_buffer(data, shoffset + elf::SH_ENTSIZE_OFFSET[arch] as usize)
    };

    return section_header;
}