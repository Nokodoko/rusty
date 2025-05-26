// struct Unprintable(i32);
//
// #[derive(Debug)]
// struct DebugPrintable(i32);

// #[derive(Debug)]
// struct Structure(i32);
//
// #[derive(Debug)]
// struct Deep(Structure);
//
// fn main() {
//     println!("{:?} months in a year", 12);
//     println!(
//         "{1:?} {0:?} is the {actor:?}, name",
//         "Slater",
//         "Christian",
//         actor = "actor's"
//     );
//
//     println!("Now {:?} will print!", Structure(3));
// }

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Peter";
    let age = 27;

    let peter = Person { name, age };

    //pretty print structs
    println!("{:#?}", peter);
}
