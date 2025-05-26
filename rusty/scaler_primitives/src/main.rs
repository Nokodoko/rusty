fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;
    // mutable = true; -- Error!

    let mutable = true;
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
}
