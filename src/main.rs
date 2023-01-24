use std::path;
use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Got target file '{}'", file_path);

    
    if path::Path::new(file_path).exists() {
        println!("File exists, reading '{}'", file_path);
        let contents: Result<Vec<u8>, std::io::Error> = fs::read(file_path);
        if contents.is_ok() {
            let bytes: &Vec<u8> = &contents.expect("");
            for byte in bytes {
                println!("{}", byte);
            }
        }
    } else {
        println!("[Error] '{}' does not exist", file_path);
        exit(-1);
    }
}
