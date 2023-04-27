// patcher.rs
// Author: Garrett Dickinson
// Created: 04/06/2023
// Description: Houses binary rewriting and patching functionality for chisel.

use std::path;
use std::fs;
use std::collections::HashMap;
use std::io::Write;
use std::io::Error;

use crate::util;
use crate::elf;


pub fn patch_binary(
    binary_contents: Vec<u8>,
    binary_name: String,
    patch_file_path: &String,
    file_header: &elf::FileHeader,
    section_header_map: HashMap<String, elf::SectionHeader>,
    note_segment: elf::ProgramHeader) {
    

    let patch_result: Result<Vec<u8>,Error> = read_patch_file(patch_file_path);
    let patch_data: &Vec<u8> = &patch_result.as_ref().unwrap();
    
    let mut program_data: Vec<u8> = binary_contents;


    // Apply patch to end of binary
    println!("Patch data read successfully, injecting at end of binary...");

    let injection_offset: usize = program_data.len();
    let injection_size: usize = patch_data.len();
    let injection_addr: usize = injection_offset;

    program_data.extend_from_slice(patch_data);

    print!("Done!");


    // Locate a note segment
    println!("Pulling .note.ABI-tag segment data...");
    let note_section: &elf::SectionHeader = section_header_map.get(".note.ABI-tag")
        .expect("[Error] Failed to pull ABI-tag section from binary!");
    print!("Done!\n");
    
    println!("Note section address: {:#04x}", note_section.addr);
    println!("Note section offset: {:#04x}", note_section.offset);
    println!("Note section size: {}", note_section.size);
    println!("");
    println!("Injected address: {:#04x}", injection_addr);
    println!("Injected section offset: {:#04x}", injection_offset);
    println!("Injected section size: {}", injection_size);


    // Rewrite the section header
    let mut injected_section: elf::SectionHeader = elf::SectionHeader::from(note_section.clone());

    injected_section.section_type = 1;
    injected_section.addr = injection_addr as u64;
    injected_section.offset = injection_offset as u64;
    injected_section.size = injection_size as u64;
    
    util::overwrite_section_header(
        &mut program_data,
        file_header.shoff as usize,
        file_header.shentsize as usize,
        injected_section.id as usize,
        &injected_section,
        file_header.is_x86_64
    );


    // 
    // Rewrite the section header
    let mut injected_segment: elf::ProgramHeader = elf::ProgramHeader::from(note_segment.clone());

    injected_segment.program_type = 1;
    injected_segment.offset = injection_offset as u64;
    injected_segment.vaddr = injection_offset as u64;
    injected_segment.paddr = injection_offset as u64;
    injected_segment.filesz = injection_size as u64;
    injected_segment.memsz = injection_size as u64;
    injected_segment.flags = 5;
    injected_segment.align = 0x1000;

    util::overwrite_segment_header(
        &mut program_data,
        file_header.shoff as usize,
        file_header.shentsize as usize,
        injected_section.id as usize,
        &injected_section,
        file_header.is_x86_64
    );




    util::overwrite_entrypoint(&mut program_data, injection_offset);


    // Spit everything back out
    let out_file_name: String = binary_name + ".patched";

    println!("Writing '{}' to disk...", out_file_name);

    let mut file = std::fs::File::create(out_file_name)
        .expect("[Error] Could not write patched binary to disk");

    file.write_all(&program_data)
        .expect("[Error] Could not write to patched binary data file");
}


fn read_patch_file(patch_path: &String) -> Result<Vec<u8>, std::io::Error> {

    if path::Path::new(patch_path).exists() && patch_path.ends_with(".bin") {
        println!("Patch file exists, reading '{}'...", patch_path);
        
        let contents: Result<Vec<u8>, std::io::Error> = fs::read(patch_path);
        
        return contents;

    } else {
        println!("[Error] Patch file '{}' is invalid or cannot be read, exiting...", patch_path);
        std::process::exit(0);    
    }
}