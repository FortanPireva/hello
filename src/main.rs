mod basics;
mod arrays;

fn main() {
    // let primes = [2,3,5,7,11,13,17,19,23,29];
    // for prime in primes {
    //     for i in 2..prime {
    //         assert_ne!(prime % i,0);
    //     }
    // }
    // let matrix = [
    //     [101, 102, 103], // <-- the comment makes rustfmt add a newline
    //     [201, 202, 203],
    //     [301, 302, 303],
    // ];

    // println!("matrix: {:#?}", matrix);
    // let transposed = arrays::transpose(matrix);
    // println!("transposed: {:#?}", transposed);

    // let a = 'A';
    // let b = 'B';
    // let mut r: &char = &a;
    // println!("r: {}", *r);
    // r = &b;
    // println!("r: {}", *r);

        // let mut point = (1,2);
        // let x_cord = &mut point.0;
        // *x_cord = 20;
        // println!("point: {point:?}");



    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    println!("s: {s:?}");
}

// Macros
// Macros are expanded into Rust code during compilation, and can take a variable number of arguments. They are distinguished by a ! at the end. The Rust standard library includes an assortment of useful macros.

// println!(format, ..) prints a line to standard output, applying formatting described in std::fmt.
// format!(format, ..) works just like println! but returns the result as a string.
// dbg!(expression) logs the value of the expression and returns it.
// todo!() marks a bit of code as not-yet-implemented. If executed, it will panic.
// unreachable!() marks a bit of code as unreachable. If executed, it will panic.