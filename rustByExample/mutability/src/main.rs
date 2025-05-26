fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mututaion {}", mutable_binding);

    // _immutable_binding += 1;
}
