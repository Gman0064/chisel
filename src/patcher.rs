// patcher.rs
// Author: Garrett Dickinson
// Created: 04/06/2023
// Description: Houses binary rewriting and patching functionality for chisel.

use std::path;

use crate::util;


pub fn patch_binary(binary_contents: &Vec<u8>, patch_file_path: &String) {
    parse_patch_file(patch_file_path);
    
    // If valid
    //      Apply patch
    // else
    //      err


}


fn parse_patch_file(patch_path: &String) {
    // Load the file from patch_binary() arg
    // Iterate through file, if line starts with # ignore
    // Otherwise, parse as such
    //      [ADDRESS] [SPACE] [HEX],[HEX],[HEX],....
    //      [ADDRESS] [SPACE] [STRING]

    if path::Path::new(patch_path).exists() && patch_path.ends_with(".patch") {
        println!("Patch file exists, reading '{}'...", patch_path);
        
        let contents = util::read_lines(patch_path.to_string());
        
        for line in contents {
            let unwrapped = line.unwrap();
            if unwrapped.trim().starts_with("#") {

            } else {
                let mut statement = unwrapped.split(":");
                let address: i32 = statement.next().unwrap().trim().parse::<i32>().unwrap();
                let data: &str = statement.next().unwrap().trim();

                if !data.is_empty() {
                    if data.contains("\"") {
                        // Value is a string literal
                        let cleaned = data.replace("\"", "");
                        let bytes = cleaned.as_bytes();
                        
                        print!("{}: ", address);
                    
                        let mut i = 0;
                        while i < bytes.len() {
                            print!("{} ", bytes[i]);
                            i = i + 1;
                        }

                        println!();
                        
                    } else {
                        // Data is comma seperated list or a single value
                        let byte_str: String = data.replace(",", "");
                        let mut bytes: Vec<u8> = util::decode_hex(&byte_str).unwrap();
                        
                        print!("{}: ", address);
                    
                        let mut i = 0;
                        while i < bytes.len() {
                            print!("{} ", bytes[i]);
                            i = i + 1;
                        }

                        println!();
                    }
                }
            }
        }
    } else {
        println!("[Error] Patch file '{}' is invalid or cannot be read, exiting...", patch_path);
        std::process::exit(0);    
    }

    std::process::exit(0);
}


fn apply_patch() {
    // Iterate through parsed patch information
    // Find the address at start of line, then apply following data to section
    // Make sure within size limits of data, otherwise boundary problems

    // Strings -> Iterate through characters specifically and parse them as individual ASCII chars
}


