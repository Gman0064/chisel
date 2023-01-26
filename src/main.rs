use std::path;
use std::env;
use std::fs;
use std::process::exit;

const ELF_MAGIC_NUMBER: &[u8] = &[0x7F,0x45,0x4C,0x46];

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
                println!("[Error] Could not find magic number, is this an executable?")
            }
        }
    } else {
        println!("[Error] '{}' does not exist", file_path);
        exit(-1);
    }

    return;
}
