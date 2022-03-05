enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

fn main() {
    let triple = (0, -2, 3);

    // `match` can be used to destructure a tuple

    match triple {
        (0, y, z) => println!("First is 0, `y` is {:?} and `z` is {:?}", y, z),
        (1, ..) => println!("First is 1, don't care about rest"),
        _ => println!("Anything else"),
    }

    let color = Color::HSV(122, 17, 40);

    // If you don't specify all the cases for each enum
    // Compiler will complain `non-exhaustive match`
    match color {
        Color::Red => println!("It's red"),
        Color::Blue => println!("It's blue"),
        Color::Green => println!("It's green"),
        // You can destructure tuple-like enums
        Color::RGB(r, g, b) => println!("Red: {} Green: {} Blue {}", r, g, b),
        Color::HSV(h, s, v) => println!("H: {} S: {} V: {}", h, s, v),
    }

    // Dereferencing: `*`
    // Destructuring: `&`, `ref`, and `ref mut`

    // A reference of type u32
    // `&` signifies that it is an reference
    let reference = &4;

    match reference {
        // `&val` also works, `ref mut val` doesn't because original was not `mut`
        ref val => println!("Got a value via destructuring {:?}", val),
    }

    match *reference {
        val => println!("Deference before case: {}", val),
    }

    let value = 5;
    match value {
        5 => println!("Hello"),
        _ => println!("Doesn't match"),
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        // You can destructure
        Foo { x: (1, b), y: 3 } => println!("Match first of x=1, second is {}, y=3", b),
        // Order doesn't matter
        Foo { y: 2, x: i } => println!("Like this x={:?}", i),

        // You can ignore some attributes
        Foo { y, .. } => println!("Y: {}, you can't use rest", y),
    }

    let pair = (2, -2);
    match pair {
        // The `if` is called `guard`
        // A `match guard` can be used to filter the `arm`
        (x, y) if x == y => println!("Twins"),
        (x, y) if x + y == 0 => println!("Antimatter"),
        _ => println!("No correlation!"),
    }

    let age = 15;
    match age {
        0 => println!("Just born"),
        // To use the variables that were used for matching,
        // use `@` sigil. It binds the value to the variable
        n @ 1..=12 => println!("Child of age: {}", n),
        n @ 13..=19 => println!("Teen of age: {}", n),
        n => println!("Adult of age: {}", n),
    }
}
