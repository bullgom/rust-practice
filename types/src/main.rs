fn main() {

    // Remember you can use underscore to make number clear?
    // You can also do that for the type specification
    let decimal = 65.4321_f32;

    // Implicit conversions are not allowed
    // Meaning conversion of types without specification is not allowed
    // let integer: u8 = decimal

    // Explicit conversions are allowed
    // Here `as` keyword is used to do that
    let integer = decimal as u8;
    let character = integer as char;

    // Some can't be directly converted
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as u16 is {}", 1000 as u16);
    // If too small error is thrown,
    // But this can be suppressed with something,
    // But rather not know that
    // println!("1000 as u8 is {}", 1000 as u8);

    // The rust compiler is pretty smart with its type inference engine
    // Yeah its got an inference engine of its own
    let elem = 5u8; // Type of `elem` is inferred from `u8` suffix

    // Vector is a growable array
    // The compiler must know what type it would hold,
    let mut vec = Vec::new();

    // It looks over and finds this
    vec.push(elem);
    // Aha! Now the compiler can infer that `vec` is for `u8`!

    println!("{:?}", vec);

    // You can alias types with `type <name> = <original-type>`
    type ID = u64;

    let id : ID = 100;
    println!("{}", id);
}
