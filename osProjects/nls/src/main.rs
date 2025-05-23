use std::process::Command;

fn main() {
    let ls = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .output()
        .expect("Unable to run nls on your directory");

        println!("{}", String::from_utf8_lossy(&ls.stdout)); 
}
