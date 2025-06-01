#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    let integer = decimal as u8;
    let character = decimal;

    println!("Casting: {} -> {} - {}", decimal, integer, character);
    println!("1000 as u16 is: {}", 1000 as u16);
}
