// util.rs
// Author: Garrett Dickinson
// Created: 02/02/2023
// Description: Utility script for storing common-use and helper
//              functions.

use std::mem;

use crate::elf::{self, EndianType, ArchitectureType};


pub fn build_file_header(data: &Vec<u8>) -> elf::FileHeader {

    // Determine x86 or x64 architecture
    // 0 : x86
    // 1 : x64
    let arch: usize = (data[elf::ARCH_OFFSET as usize] - 1).into();

    let file_header: elf::FileHeader = elf::FileHeader {
        arch: parse_architecture(data[elf::ARCH_OFFSET as usize]),
        is_x86_64: arch != 0,
        endian: parse_endian(data[elf::ENDIAN_OFFSET as usize]),
        abi: data[elf::ABI_OFFSET as usize],
        abi_str: parse_abi(data[elf::ABI_OFFSET as usize]),
        elf_type: u16_from_buffer(data, elf::TYPE_OFFSET as usize),
        isa: u16_from_buffer(data, elf::MACHINE_OFFSET as usize),
        isa_str: parse_isa(u16_from_buffer(data, elf::MACHINE_OFFSET as usize)),
        entryoff: u64_from_buffer(data, elf::ENTRYPOINT_OFFSET as usize),
        phoff: u64_from_buffer(data, elf::PHOFF_OFFSET[arch] as usize),
        shoff: u64_from_buffer(data, elf::SHOFF_OFFSET[arch] as usize),
        ehsize: u16_from_buffer(data, elf::EHSIZE_OFFSET[arch] as usize),
        phentsize: u16_from_buffer(data, elf::PHENTSIZE_OFFSET[arch] as usize),
        phnum: u16_from_buffer(data, elf::PHNUM_OFFSET[arch] as usize),
        shentsize: u16_from_buffer(data, elf::SHENTSIZE_OFFSET[arch] as usize),
        shnum: u16_from_buffer(data, elf::SHNUM_OFFSET[arch] as usize),
        shstrndx: u16_from_buffer(data, elf::SHSTRNDX_OFFSET[arch] as usize),
    };

    return file_header;
}


pub fn build_program_header(data: &Vec<u8>, phoffset: usize, is_x86_64: bool) -> elf::ProgramHeader {

    // Cast the supplied is_x86_64 bool to an array offset
    // 0 : x86
    // 1 : x64
    let arch: usize = is_x86_64.into();

    let program_header: elf::ProgramHeader = elf::ProgramHeader {
        program_type: u32_from_buffer(data, phoffset + elf::PH_TYPE_OFFSET as usize),
        flags: u32_from_buffer(data, phoffset + elf::PH_FLAGS_OFFSET[arch] as usize),
        offset: u64_from_buffer(data, phoffset + elf::PH_OFFSET_OFFSET[arch] as usize),
        vaddr: u64_from_buffer(data, phoffset + elf::PH_VADDR_OFFSET[arch] as usize),
        paddr: u64_from_buffer(data, phoffset + elf::PH_PADDR_OFFSET[arch] as usize),
        filesz: u64_from_buffer(data, phoffset + elf::PH_FILESZ_OFFSET[arch] as usize),
        memsz: u64_from_buffer(data, phoffset + elf::PH_MEMSZ_OFFSET[arch] as usize),
        align: u64_from_buffer(data, phoffset + elf::PH_ALIGN_OFFSET[arch] as usize)
    };

    return program_header;
}


pub fn build_section_header(data: &Vec<u8>, shoffset: usize, is_x86_64: bool) -> elf::SectionHeader {

    // Cast the supplied is_x86_64 bool to an array offset
    // 0 : x86
    // 1 : x64
    let arch: usize = is_x86_64.into();

    let section_header: elf::SectionHeader = elf::SectionHeader {
        name: u32_from_buffer(data, shoffset + elf::SH_NAME_OFFSET as usize),
        section_type: u32_from_buffer(data, shoffset + elf::SH_TYPE_OFFSET as usize),
        flags: u64_from_buffer(data, shoffset + elf::SH_FLAGS_OFFSET as usize),
        addr: u64_from_buffer(data, shoffset + elf::SH_ADDR_OFFSET[arch] as usize),
        offset: u64_from_buffer(data, shoffset + elf::SH_OFFSET_OFFSET[arch] as usize),
        size: u64_from_buffer(data, shoffset + elf::SH_SIZE_OFFSET[arch] as usize),
        link: u32_from_buffer(data, shoffset + elf::SH_LINK_OFFSET[arch] as usize),
        info: u32_from_buffer(data, shoffset + elf::SH_INFO_OFFSET[arch] as usize),
        addralign: u64_from_buffer(data, shoffset + elf::SH_ADDRALIGN_OFFSET[arch] as usize),
        entsize: u64_from_buffer(data, shoffset + elf::SH_ENTSIZE_OFFSET[arch] as usize)
    };

    return section_header;
}


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
        0x00 => "System V".to_string(),
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

    let result = String::from_utf8(name).expect("[Error] Failed to parse section name!");
    
    return result;
}


pub fn parse_elf_type(elf_type: u16) -> String {
    match elf_type {
        0x0000 => "Unknown".to_string(),
        0x0001 => "Relocatable file".to_string(),
        0x0002 => "Executable file".to_string(),
        0x0003 => "Shared object".to_string(),
        0x0004 => "Core file".to_string(),
        0xFE00 => "Reserved, operating system specific".to_string(),
        0xFEFF => "Reserved, operating system specific".to_string(),
        0xFF00 => "Reserved, processor specific".to_string(),
        0xFFFF => "Reserved, processor specific".to_string(),

        // Match unknown segment type
        _ => "UNKNOWN".to_string()
    }
}


pub fn parse_section_type(section_type: u32) -> String {
    match section_type {
        0x00000000 => "SHT_NULL".to_string(),
        0x00000001 => "SHT_PROGBITS".to_string(),
        0x00000002 => "SHT_SYMTAB".to_string(),
        0x00000003 => "SHT_STRTAB".to_string(),
        0x00000004 => "SHT_RELA".to_string(),
        0x00000005 => "SHT_HASH".to_string(),
        0x00000006 => "SHT_DYNAMIC".to_string(),
        0x00000007 => "SHT_NOTE".to_string(),
        0x00000008 => "SHT_NOBITS".to_string(),
        0x00000009 => "SHT_REL".to_string(),
        0x0000000A => "SHT_SHLIB".to_string(),
        0x0000000B => "SHT_DYNSYM".to_string(),
        0x0000000E => "SHT_INIT_ARRAY".to_string(),
        0x0000000F => "SHT_FINI_ARRAY".to_string(),
        0x00000010 => "SHT_PREINIT_ARRAY".to_string(),
        0x00000011 => "SHT_GROUP".to_string(),
        0x00000012 => "SHT_SYMTAB_SHNDX".to_string(),
        0x00000013 => "SHT_NUM".to_string(),
        0x60000000 => "SHT_LOOS".to_string(),

        // Match unknown segment type
        _ => "UNKNOWN".to_string()
    }
}


pub fn parse_section_flags(flags: u64) -> String {
    match flags {
        0x00000001 => "SHF_WRITE".to_string(),
        0x00000002 => "SHF_ALLOC".to_string(),
        0x00000004 => "SHF_EXECINSTR".to_string(),
        0x00000010 => "SHF_MERGE".to_string(),
        0x00000020 => "SHF_STRINGS".to_string(),
        0x00000040 => "SHF_INFO_LINK".to_string(),
        0x00000080 => "SHF_LINK_ORDER".to_string(),
        0x00000100 => "SHF_OS_NONCONFORMING".to_string(),
        0x00000200 => "SHF_GROUP".to_string(),
        0x00000400 => "SHF_TLS".to_string(),
        0x0FF00000 => "SHF_MASKOS".to_string(),
        0xF0000000 => "SHF_MASKPROC".to_string(),
        0x40000000 => "SHF_ORDERED".to_string(),
        0x80000000 => "SHF_EXCLUDE".to_string(),

        // Match unknown segment type
        _ => "UNKNOWN".to_string()
    }
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


pub fn pp_file_header(header: &elf::FileHeader) {
    println!("- Architecture: {:?}", header.arch);
    println!("- Endian: {:?}", header.endian);
    println!("- ABI: {}", header.abi_str);
    println!("- Type: {}", parse_elf_type(header.elf_type));
    println!("- ISA: {}", header.isa_str);
    println!("- Entry Point: {:#04x}", header.entryoff);
    println!("- Program Offset: {:#04x}", header.phoff);
    println!("- Program Entry Size: {}", header.phentsize);
    println!("- Number Program Entries: {}", header.phnum);
    println!("- Section Offset: {:#04x}", header.shoff);
    println!("- Section Entry Size: {}", header.shentsize);
    println!("- Number Section Entries: {}", header.shnum);
}


pub fn pp_section_header(header: &elf::SectionHeader, number: i32, name: &String) {
    println!("[{}] {}", number, name);
    println!("\t- Type: {}", parse_section_type(header.section_type));
    println!("\t- Flags: {}", parse_section_flags(header.flags));
    println!("\t- Address: {:#04x}", header.section_type);
    println!("\t- Offset: {:#04x}", header.section_type);
    println!("\t- Link Index: {}", header.link);
    println!("\t- Info Bytes: {}", header.info);
    println!("\t- Alignment: {}", header.info);
    println!();
}


pub fn pp_program_header(header: &elf::ProgramHeader, number: i32, ph_type: &String) {
    println!("[{}] {}", number, ph_type);
    println!("\t- Type: {}", parse_section_type(header.program_type));
    println!("\t- Flags: {}", header.flags);
    println!("\t- Offset: {:#04x}", header.offset);
    println!("\t- Virtual Address: {:#04x}", header.vaddr);
    println!("\t- Physical Address: {:#04x}", header.paddr);
    println!("\t- File Size: {}", header.filesz);
    println!("\t- Memory Size: {}", header.memsz);
    println!("\t- Alignment: {}", header.align);
    println!();
}

