use std::fs;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
   fs::read_to_string("username.txt")
}


pub fn main() {
    let username = read_username_from_file().unwrap_or_else(|e| {"World".to_string()});
    println!("Hello {username}");
}