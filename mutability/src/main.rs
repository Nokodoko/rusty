fn main() {
    let mut x = 5;
    println!("value of x is: {x}");
    x = 6;
    println!("value of x is: {x}");

    let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("Index of 3 is {} and index 4 is {}", a[3], a[5]); 

    let b: f64 = 3.1;
    println!("b = {}", b); 

    let words = "test String";
    println!("{}", words); 
}
