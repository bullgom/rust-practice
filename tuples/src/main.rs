// Import via `use` the `fmt` module
use std::fmt;

// You don't need definition?
// To use the `{}` marker, the trait `fmt::Display` must be implemented
impl fmt::Display for Matrix {
    // Trait requires `fmt` with this exact signature
    // Try commenting the following function
    // Above `impl` will be sqwiggled saying you need `fmt(some signature)`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // f is the stream
        // self is the target struct
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

/*
 * Tuple is a collection of values.
 * The values may have different type.
 * Constructed with parenthesis `()`
 * Has type signature `(T1, T2, ...)`, where T_n are the types of its members
 */

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (a, b) = pair;

    // Don't get fooled. Put no semicolon if you want to return.
    (b, a)
}
// To learn more about above `Put no semicolon` check below
// https://stackoverflow.com/questions/70490809/without-return-tuple-is-recognized-as-just-parenthesis

// A named tuple style struct
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn transpose(matrix: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = matrix;

    Matrix(d, c, b, a)
}

fn main() {
    let pair = (1, true);
    println!("Pair: {:?}", pair);
    println!("Reversed: {:?}", reverse(pair));

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using `tuple indexing`
    println!("First value of long_tuple is: {}", long_tuple.0);
    println!("Second value of long_tuple is: {}", long_tuple.1);

    // Since tuple can hold multiple types, it can also hold tuples
    let nested_tuples = ((1, 2, 3), (3, 2, 1), (1, 1));
    println!("Nested {:?}", nested_tuples);
    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error
    /*
     * Did it: Throws an error saying `Cannot be formatted using {:?}
     * because it doesn't implement `Debug`
     */

    // Like python, to create size-1 tuple, a comma is required
    println!("A tuple: {:?}", (32,));
    println!("An int : {:?}", (32));

    // A tuple can be destructured (Not destructed)
    let future_destructured = (1, "helo", 4.5, true);
    let (a, b, c, d) = future_destructured;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix\n{:}", matrix);
    let transposed = transpose(matrix);
    println!("Transposed\n{:}", transposed);
}
