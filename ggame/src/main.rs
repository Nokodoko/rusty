use std::io;

fn main() {
    println!("This is a guessing game");
    println!("Input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess {guess}");
}
