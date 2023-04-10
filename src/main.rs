// main.rs
// Author: Garrett Dickinson
// Created: 01/23/2023
// Description: Main entrypoint script for chisel. Contains basic procedures
//              for gathering ELF file and program data.

use iced_x86::{Decoder, DecoderOptions, Formatter, NasmFormatter, Instruction};
use std::collections::HashMap;
use std::path;
use std::env;
use std::fs;
use std::process::exit;

// Import modules
mod elf;
mod util;
mod patcher;


fn main() {
    // Collect our execution args
    let args: Vec<String> = env::args().collect();
    let mut patch_mode: bool = false;
    let mut patch_file_path: &String = &"".to_string();

    // Grab our filepath from our options
    if &args.len() < &2 {
        // No file given, terminate
        util::print_help();
        exit(0);
    }

    // Check if the arguments we have include the patching flag and file
    if &args.len() > &2 {
        if &args[2] == "-p" {
            if &args.len() >= &4 {
                patch_mode = true;
                patch_file_path = &args[3];
            } else {
                util::print_help();
                exit(0);    
            }
        } else {
            // More than 1 arg but no patching flag given, terminate
            util::print_help();
            exit(0);
        }
    }

    let file_path: &String = &args[1];
    
    if path::Path::new(file_path).exists() {
        println!("File exists, reading '{}'...", file_path);
        
        let contents: Result<Vec<u8>, std::io::Error> = fs::read(file_path);
        
        if contents.is_ok() {

            let bytes: &Vec<u8> = &contents.expect("");
            let magic_num: &[u8] = &bytes[0..4];
        
            // Check to see if our file contains the ELF magic number
            if magic_num == elf::MAGIC_NUMBER {

                println!("Found ELF Magic Number...");
                println!("Parsing File Header...");

                // Build the File Header data structure
                let file_header: elf::FileHeader = util::build_file_header(bytes);

                println!("\t- Found {} program header entries {} bytes in length", file_header.phnum, file_header.phentsize);
                println!("\t- Found {} section header entries {} bytes in length", file_header.shnum, file_header.shentsize);
                println!("\t- Found .shstrtab section at index {}", file_header.shstrndx); 

                println!("\n==== File Header ====");
                util::pp_file_header(&file_header);


                println!("\nParsing Section Headers...");

                // Determine the shstrtab offset.
                // This is found by taking the string table index and multiplying it by the section header entry size, then
                // adding this to the initial section header offset.
                let shstrtab_offset: u64 = file_header.shoff + (file_header.shentsize as u64 * file_header.shstrndx as u64);

                // Build a read-only version of the .shstrtab section
                let shstrtab_section: elf::SectionHeader = util::build_section_header(
                    bytes,
                    shstrtab_offset as usize,
                    file_header.is_x86_64
                );

                // Define all of our offsets for the shstrtab, and build a u8 buffer of the data
                let shstrtab_start: u64 = shstrtab_section.offset;
                let shstrtab_end: u64 = shstrtab_section.offset + shstrtab_section.size;
                let shstrtab_data: Vec<u8> = bytes[shstrtab_start as usize..shstrtab_end as usize].to_vec();

                println!("\t- Found .shstrtab section");

                println!("\n==== Sections ====");

                let mut section_table_map: HashMap<String, elf::SectionHeader> = HashMap::new();
                let mut section_table_offset: u64 = file_header.shoff;
                let mut section_table_count: i32 = 0;

                // Iterate through number of section headers
                for _ in 0..file_header.shnum {

                    // Build section header data structure
                    let section_header: elf::SectionHeader = util::build_section_header(
                        bytes, 
                        section_table_offset as usize,
                        file_header.is_x86_64
                    );

                    // Determine the section name for each section using the shstrtab data
                    let section_name: String = util::parse_section_name(&shstrtab_data, section_header.name as usize);

                    util::pp_section_header(&section_header, section_table_count, &section_name);

                    section_table_map.insert(section_name, section_header);

                    // Update the section table offset counter based on the section header size
                    section_table_offset += file_header.shentsize as u64;
                    section_table_count += 1;
                }

                println!("\nParsing Program Segments...");
                
                println!("\n==== Program Segments ====");

                let mut program_table_offset = file_header.phoff;
                let mut program_table_count: i32 = 0;

                // Iterate through number of Program Headers
                for _ in 0..file_header.phnum {
                    // Build Program Header data structure
                    let program_header: elf::ProgramHeader = util::build_program_header(
                        bytes, 
                        program_table_offset as usize,
                        file_header.is_x86_64
                    );

                    // Parse the program name using the program type
                    let program_name: String = util::parse_program_segment_type(program_header.program_type);

                    util::pp_program_header(&program_header, program_table_count, &program_name);

                    // Update the program header table offset counter based on the program header size
                    program_table_offset += file_header.phentsize as u64;
                    program_table_count += 1;
                }


                // Now that we have all the sections, spit out the .text section and start a linear disassembly
                let text_section: &elf::SectionHeader = section_table_map.get(".text").unwrap();
                let text_section_offset: usize = text_section.offset as usize;
                let text_section_end: usize = text_section_offset + text_section.size as usize;

                // Buffer of text section data
                let text_section_buff: &[u8] = &bytes[text_section_offset..text_section_end];

                // Offsets for resizing buffer
                let mut instr_start: usize = 0;
                let mut instr_end: usize = 1;
                let mut ip_offset: u64 = text_section.offset;

                // Define our instruction buffer
                let mut instr_bytes: &[u8] = &text_section_buff[instr_start..instr_end];

                // Define our decoder and icedx86 variables
                let mut decoder: Decoder = Decoder::with_ip(64, instr_bytes, text_section.offset, DecoderOptions::NONE);
                let mut formatter: NasmFormatter = NasmFormatter::new();
                let mut instruction: Instruction = Instruction::default();
                let mut output = String::new();

                // Specify options for our NASM instruction formatter
                // Formatting and linear sweep pattern partially borrowed from icedx86 docs
                // https://docs.rs/iced-x86/latest/iced_x86/#disassemble-decode-and-format-instructions
                formatter.options_mut().set_digit_separator("`");
                formatter.options_mut().set_first_operand_char_index(10);

                
                println!("==== Text Section Analysis ====\n");

                while decoder.can_decode() {
                    // Decode the instruction sub-buffer
                    decoder.decode_out(&mut instruction);
            
                    if instruction.is_invalid() {
                        
                        // Instruction invalid
                        // Increase the buffer size by 1, then try to parse again
                        instr_end = instr_end + 1;
                        instr_bytes = &text_section_buff[instr_start..instr_end];
                        decoder = Decoder::with_ip(64, instr_bytes, ip_offset, DecoderOptions::NONE);

                    } else {

                        // Got a valid instruction
                        // Format the instruction for printing
                        output.clear();
                        formatter.format(&instruction, &mut output);

                        // Print the instruction to an output assembly file
                        println!("{:016X}\t{}", instruction.ip(), output);

                        // Reset the buffer start to the end of the previous buffer, then try to parse again
                        instr_start = instr_end;

                        // If the instruction end index is less than the buffer, increase it
                        if (instr_end + 1) < text_section_buff.len() {
                            instr_end = instr_end + 1;
                        }
                    
                        // Resize the instruction buffer and parse it again
                        instr_bytes = &text_section_buff[instr_start..instr_end];
                        ip_offset = text_section.offset + instr_start as u64;
                        decoder = Decoder::with_ip(64, instr_bytes, ip_offset, DecoderOptions::NONE);
                    }
                }


                if patch_mode {

                    println!("\n==== Applying Patch To Binary ====\n");

                    patcher::patch_binary(
                        bytes.to_vec(),
                        file_path.to_string(),
                        &patch_file_path
                    );
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