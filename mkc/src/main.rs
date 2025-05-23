use std::io::Read;
use std::io::Result;
use std::fs::File;
use std::string::String;

fn main() -> Result<()> {
    let mut file = File::open("/home/n0ko/fontFix.md").expect("Unable to open file");
    let mut content = vec![];

    file.read_to_end(&mut content).expect("Unable to read file");
    println!("{}", String::from_utf8_lossy(&content));
    Ok(())
}
