use std::io::{Stdin, self};
use anyhow::{Result, Error};

pub fn greet_user() -> Result<String>{
    println!("Please type your name:"); 
    let mut buffer: String = String::new();
    let stdin: Stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    if buffer.trim().to_lowercase() != "n0ko" {
        Err(Error::msg{message:"Denied"});
    } else {
        Ok(buffer);
    }
    buffer
}

