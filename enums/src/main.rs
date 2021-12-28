// Struct is also valid as enum

enum WebEvent {
    // An enum value may either be
    // 1. unit-like
    PageLoad, // 0
    PageUnload, // 1

    // 2. tuple-like structs
    KeyPress(char), // ...
    Paste(String),

    // 3. c-like structs
    Click { x: i64, y: i64 },
}
// Note above enum is created with implicit discriminator meaning that it starts at 0

// You can specify the discriminator explicitly
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the enum
        WebEvent::KeyPress(c) => println!("Pressed `{}`", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),

        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned version
    // Like a copy()
    // Instead `&` is borrow version, like a reference
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    /* 2nd part */
    // Use statement can be used to ommit the manual scoping
    use crate::WebEvent::PageLoad;
    // Now you can use PageLoad without the scoping
    inspect(PageLoad);

    // You can make everything in the scope open with a star
    use crate::WebEvent::*;

    inspect(PageUnload);

    // `enums` can be cast as integers
    println!("Roses are #{:06x}", Color::Red as i32);
}
