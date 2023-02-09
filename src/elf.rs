// elf.rs
// Author: Garrett Dickinson
// Created: 02/02/2023
// Description: Provides constant and struct definitions for data
//              structures related to the ELF binary specification


///////////////////////////////////////////////////////////////////////////////
/// 
/// Generic ELF information offsets.
/// 
///////////////////////////////////////////////////////////////////////////////

pub const MAGIC_NUMBER: &[u8] = &[0x7F,0x45,0x4C,0x46];
pub const ARCH_OFFSET: u8 = 0x04;       // x86 or x64 indiicator; 1 byte
pub const ENDIAN_OFFSET: u8 = 0x05;     // Endian offset (1 - little, 2 - big); 1 byte
pub const ABI_OFFSET: u8 = 0x07;        // ABI identifier; 1 byte
pub const TYPE_OFFSET: u8 = 0x10;       // Object type identifier; 2 bytes
pub const MACHINE_OFFSET: u8 = 0x12;    // Instruction set type; 2 bytes


///////////////////////////////////////////////////////////////////////////////
///
/// Offsets for file header entry points and table information.
/// Arrayed offset are split by architecture:
///      0 : x86
///      1 : x86_64
/// 
///////////////////////////////////////////////////////////////////////////////

pub const ENTRYPOINT_OFFSET: u8 = 0x18;
pub const PHOFF_OFFSET: [u8; 2] = [0x1C, 0x20];        // Program header table pointer; 2 bytes
pub const SHOFF_OFFSET: [u8; 2] = [0x20, 0x28];        // Section table pointer; 2 bytes
pub const EHSIZE_OFFSET: [u8; 2] = [0x28, 0x34];       // Program header table entry size pointer; 2 bytes
pub const PHENTSIZE_OFFSET: [u8; 2] = [0x28, 0x34];    // Section table pointer; 2 bytes
pub const PHNUM_OFFSET: [u8; 2] = [0x2C, 0x38];        // Program header table number of entries pointer; 2 bytes
pub const SHENTSIZE_OFFSET: [u8; 2] = [0x2E, 0x3A];    // Size of section header table; 2 bytes
pub const SHNUM_OFFSET: [u8; 2] = [0x30, 0x3C];        // Number of entries in section table pointer; 2 bytes
pub const SHSTRNDX_OFFSET: [u8; 2] = [0x32, 0x3E];     // Index of section header that contains names; 2 bytes


///////////////////////////////////////////////////////////////////////////////
///
/// Offsets for program header information.
/// Arrayed offset are split by architecture:
///      0 : x86
///      1 : x86_64
/// 
///////////////////////////////////////////////////////////////////////////////

pub const PH_TYPE_OFFSET: u8 = 0x00;
pub const PH_FLAGS_OFFSET: [u8; 2] = [0x18, 0x04];
pub const PH_OFFSET_OFFSET: [u8; 2] = [0x04, 0x08];
pub const PH_VADDR_OFFSET: [u8; 2] = [0x08, 0x10];
pub const PH_PADDR_OFFSET: [u8; 2] = [0x0C, 0x18];
pub const PH_FILESZ_OFFSET: [u8; 2] = [0x10, 0x20];
pub const PH_MEMSZ_OFFSET: [u8; 2] = [0x14, 0x28];
pub const PH_ALIGN_OFFSET: [u8; 2] = [0x1C, 0x30];


///////////////////////////////////////////////////////////////////////////////
///
/// Offsets for section header information.
/// Arrayed offset are split by architecture:
///      0 : x86
///      1 : x86_64
/// 
///////////////////////////////////////////////////////////////////////////////

pub const SH_NAME_OFFSET: u8 = 0x00;
pub const SH_TYPE_OFFSET: u8 = 0x04;
pub const SH_FLAGS_OFFSET: u8 = 0x08;
pub const SH_ADDR_OFFSET: [u8; 2] = [0x0C, 0x10];
pub const SH_OFFSET_OFFSET: [u8; 2] = [0x10, 0x18];
pub const SH_SIZE_OFFSET: [u8; 2] = [0x14, 0x20];
pub const SH_LINK_OFFSET: [u8; 2] = [0x18, 0x28];
pub const SH_INFO_OFFSET: [u8; 2] = [0x1C, 0x2C];
pub const SH_ADDRALIGN_OFFSET: [u8; 2] = [0x20, 0x30];
pub const SH_ENTSIZE_OFFSET: [u8; 2] = [0x24, 0x38];



#[derive(Debug)]
pub enum ArchitectureType {
    X86,
    X86_64,
    Unknown
}


#[derive(Debug)]
pub enum EndianType {
    Big,
    Little,
    Unknown
}

// TODO: Types in structs for holding addresses are most likely
//       too small, increase to u32 maybe?
//       Refer to structs in /usr/include/elf.h for this

#[derive(Debug)]
pub struct FileHeader {
    pub arch: ArchitectureType,
    pub is_x86_64: bool,
    pub endian: EndianType,
    pub abi: u8,
    pub elf_type: u8,
    pub isa: u8,
    pub entryoff: u8,
    pub phoff: u8,
    pub shoff: u8,
    pub ehsize: u8,
    pub phentsize: u8,
    pub phnum: u8,
    pub shentsize: u8,
    pub shnum: u8,
    pub shstrndx: u8
}


#[derive(Debug)]
pub struct ProgramHeader {
    pub program_type: u8,
    pub flags: u8,
    pub offset: u8,
    pub vaddr: u8,
    pub paddr: u8,
    pub filesz: u8,
    pub memsz: u8,
    pub align: u8,
}


#[derive(Debug)]
pub struct SectionHeader {
    pub name: u8,
    pub section_type: u8,
    pub flags: u8,
    pub addr: u8,
    pub offset: u8,
    pub size: u8,
    pub link: u8,
    pub info: u8,
    pub addralign: u8,
    pub entsize: u8
}