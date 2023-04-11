// patcher.rs
// Author: Garrett Dickinson
// Created: 04/06/2023
// Description: Houses binary rewriting and patching functionality for chisel.

use std::path;
use std::collections::HashMap;
use std::io::Write;

use crate::util;


pub fn patch_binary(binary_contents: Vec<u8>, binary_name: String, patch_file_path: &String) {
    
    let patch_data: HashMap<usize, Vec<u8>> = parse_patch_file(patch_file_path);
    let mut bytes: Vec<u8> = binary_contents;

    println!("Patch data read successfully, applying...");

    for patch in patch_data {
        let start_offset = patch.0;
        let mut i: usize = 0;
        while i < (patch.1.len()) {
            bytes[(start_offset + i) as usize] = patch.1[i];
            i += 1;
        }
    }

    println!("Done!");

    let patched_file_name: String = binary_name + "_patched";

    println!("Writing '{}' to disk...", patched_file_name);

    let mut file = std::fs::File::create(patched_file_name)
        .expect("[Error] Could not write patched binary to disk");

    file.write_all(&bytes)
        .expect("[Error] Could not write to patched binary file");

}


fn parse_patch_file(patch_path: &String) -> HashMap<usize, Vec<u8>>{
    // Load the file from patch_binary() arg
    // Iterate through file, if line starts with # ignore
    // Otherwise, parse as such
    //      [ADDRESS] [SPACE] [HEX],[HEX],[HEX],....
    //      [ADDRESS] [SPACE] [STRING]

    if path::Path::new(patch_path).exists() && patch_path.ends_with(".patch") {
        println!("Patch file exists, reading '{}'...", patch_path);
        
        let contents = util::read_lines(patch_path.to_string());
        let mut patch_data: HashMap<usize, Vec<u8>> = HashMap::new();
        
        for line in contents {
            let unwrapped = line.unwrap();
            if unwrapped.trim().starts_with("#") || unwrapped.is_empty() {
                //Skip
            } else {
                let mut statement = unwrapped.split(":");
                let address: usize = util::hex_to_int(statement.next().unwrap().trim()).unwrap();
                let data: &str = statement.next().unwrap().trim();

                if !data.is_empty() {
                    if data.contains("\"") {
                        // Value is a string literal
                        let cleaned = data.replace("\"", "");
                        let bytes: Vec<u8> = cleaned.as_bytes().to_vec();
                        
                        print!("{}: ", address);
                    
                        let mut i = 0;
                        while i < bytes.len() {
                            print!("{} ", bytes[i]);
                            i = i + 1;
                        }

                        println!();

                        patch_data.insert(address, bytes);
                        
                    } else {
                        // Data is comma seperated list or a single value
                        let byte_str: String = data.replace(",", "");
                        let bytes: Vec<u8> = util::hex_to_buff(&byte_str).unwrap();
                        
                        print!("{}: ", address);
                    
                        let mut i = 0;
                        while i < bytes.len() {
                            print!("{} ", bytes[i]);
                            i = i + 1;
                        }

                        println!();

                        patch_data.insert(address, bytes);
                    }
                }
            }
        }

        return patch_data;

    } else {
        println!("[Error] Patch file '{}' is invalid or cannot be read, exiting...", patch_path);
        std::process::exit(0);    
    }
}