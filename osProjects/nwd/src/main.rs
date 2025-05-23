use::std::process::Command;

fn main() {
    let command = Command::new("pwd")
        .output()
        .expect("Unable to print current directory");

        println!("{}", String::from_utf8_lossy(&command.stdout)); 
}
