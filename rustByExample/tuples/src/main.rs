use core::fmt;
use std::fmt::Formatter;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
            ( {} {} )
            ( {} {} )",
            self.0, self.1, self.2, self.3,
        )
    }
}

fn transpose(matrix: &Matrix) -> Matrix {
    let zero = matrix.0;
    let one = matrix.1;
    let two = matrix.2;
    let three = matrix.3;
    Matrix(three, two, one, zero)
}

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8, -2i16));

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    //NOTE: Seems to work -- maybe rust increased the size of a tuple that will print?
    let too_long_tuple = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    println!("Too long tuple: {:?}", too_long_tuple);

    let pair: (i32, bool) = (1, true);
    println!("Pair is {:?}", pair);
    println!("The reverse pair is {:?}", reverse(pair));

    println!("One element tuple: {:?}", (5u32));
    println!("Just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("{}", transpose(&matrix));
}
