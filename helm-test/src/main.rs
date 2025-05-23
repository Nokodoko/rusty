use std::io::{self, BufRead};
use std::process::Command;

fn ht(args: &Vec<String>) {
    let mut command = Command::new("helm");
    command.arg("lint");

    for arg in args {
        command.arg("-f").arg(arg);
    }

    let output = command.output().expect("Failed to execute command");
    match output.stdout.is_empty() {
        true => {}
        false => println!("stdout: {}", String::from_utf8_lossy(&output.stdout)),
    }

    match output.stderr.is_empty() {
        true => {}
        false => println!("stderr: {}", String::from_utf8_lossy(&output.stderr)),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut files: Vec<String> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    ht(&mut files);
}
