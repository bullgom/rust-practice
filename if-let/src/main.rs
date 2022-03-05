fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if-let` does
    // first, destructures the given variable
    // second, match condition

    // Destructure (Take out) the value as variable `i` from `number`
    if let Some(i) = number {
        println!("Matched {:?}", i);
    } else {
        // You can specify when destructuring fails with `else`
        println!("Didn't match");
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 3 {
            println!("Quiting");
            optional = None;
        }else {
            println!("Running! {}", i);
            optional = Some(i + 1);
        }
    }
}
