use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use assembler::assembler;
use log::{info, warn};
mod assembler;
fn read_file(file_name: String) -> std::io::Result<Vec<String>> {
    let mut list: Vec<String> = Vec::new();
    let file = File::open(file_name).expect("File not found");
    let buf = BufReader::new(file);

    for line in buf.lines() {
        list.push(line?);
    }
    Ok(list)
}
fn main() {
    let lines = read_file(String::from("test.lc3")).unwrap();
    let binary = assembler(lines);
}
