fn main() {
    //let mut values = vec![1, 2, 3, 4, 5];
    let mut values = vec![1, 2, 3, 4, 5];

    for value in &values {
        println!("value = {}", value);
    }

    if values.len() >= 5 {
        println!("List is larger than or equal to  5");
    }

    match values.len() {
        0 => println!("nil"),
        1 => println!("Leo"),
        2 => println!("Raph"),
        3 => println!("Donny"),
        4 => println!("Mikey"),

        5 => println!("Splinter"),
        _ => println!("nil"),
        //still looking for a use case where i don't need `_` as a catch-all
    }

    while let Some(value) = values.pop() {
        println!("value={value}");
    }
}
