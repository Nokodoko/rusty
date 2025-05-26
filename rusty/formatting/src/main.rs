fn main() {
    println!("{} days", 31);
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!("Base 10:  {}", 69420);
    println!("Base 2 (binary):  {:b}", 69420);
    println!("Base 8 (octal):  {:o}", 69420);
    println!("Base 16 (hexidecimal):  {:x}", 69420);

    println!("{number:>5}", number = 1); //pad with spaces
    println!("{number:0>5}", number = 1); //pad with 0's

    println!("{number:0>width$}", number = 1, width = 7);

    println!("My name is {0}, {1}, {0}", "Bond", "James");

    // #[allow(dead_code)]
    // struct Structure(i32);
    //
    // println!("This struct `{}` won't print ... ", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;

    println!("{number:>width$}");
}
