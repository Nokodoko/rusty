use core::fmt;
use std::any::Any;
use std::fmt::write;
use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element fo the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5]; // these are arrays
    let ys: [i32; 500] = [0; 500];

    println!("First element fo the slice: {}", xs[0]);
    println!("Second element fo the slice: {}", xs[1]);

    println!("The number of elements of the array {}", xs.len());

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrow whole array as a slice.");
    analyze_slice(&xs);

    println!("The borrow section fo the array as a slice");
    analyze_slice(&ys[1..4]);

    let empty_array: [i32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("slow down! {} is too far!", i),
        }

        println!("{}", xs[5]);
        println!("{}", xs[..][5]);
    }
}

// #[derive(Debug)]
// struct T {
//     define: Box<dyn Any>,
//     len: usize,
// }
//
// impl Clone for T {
//     fn clone(&self) -> Self {
//         self.define;
//         self.len;
//     }
// }

// fn analyze_array<T: Default>(array_len: usize) {
//     let mut my_array: Vec<T> = vec![Default::default(); array_len];
//     println!("First element of the slice: {:?}", my_array.get(0));
//     println!("The slice has {:?} elements", my_array.len());
// }

//     let mut my_array: Vec<T> = vec![Default::default(), array_len];
//     println!(
//         "First element of array: {}\nThe length of array is: {}",
//         my_array.get(0),
//         my_array.len()
//     );
// }
