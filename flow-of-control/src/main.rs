fn main() {
    // Branching is similar to other languages
    // Difference:
    //  1. Doesn't need parenthesis for the boolean condition
    //  2. Condition is followed by a block
    //  3. All branches must return the same type

    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");

        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };
    println!("{} -> {}", n, big_n);

    // Looping!
    let mut i = 0u32;
    loop {
        i += 1;
        if i == 3 {
            println!("Three!");

            // `continue` skips the rest of the loop
            continue;
        }

        println!("{}", i);

        if i == 5 {
            // `break` exits the loop anyhow
            break;
        }
    }

    'outer: loop {
        println!("Entered outer loop");

        'inner: loop {
            println!("Entered inner loop");

            //break; Like all other lang, this only breaks the inner most loop

            break 'outer; // This break cascades until the outer loop!
            println!("Still inside");
        }

        println!("This won't be printed");
    }

    // You can choose which value for the loop to be evaluated to!
    // Awsome!
    i = 0;

    let result = loop {
        i += 1;
        if i == 10 {
            break i * 2;
        }
    }; // Always remember to put this, when you want to evaluate the block

    println!("Result: {}", result);

    i = 0;
    while i <= 100 {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        }

        i += 1;
    }

    // For iterates through an iterator
    // One of ways to make an iterator the easiest: range notation
    // `a..b` which `yields` values from [a,b) with step 1
    // `a..=b` which '' from [a, b]

    // FizzBuzz using for
    for n in 0..=100 {
        if n % 15 == 0 {
            println!("Fizz BUZZ{}", n);
        }
    }

    // 
    let names = vec!["Bob", "Frank", "Ferris"];

    // By default `for` will use `into_iter` to the collection
    // But you can specify from 3 choices
    // `iter` - Borrows each element of the collection through each iteration, 
    //          Leaves the collection untouched
    for name in names.iter() {
        // name has type &&str
        match name {
            // expected &str, found str
            // "Ferris" => println!("There is a rustacean among us!"),
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // `into_iter` - Consumes the collection into iterators
    // So after `into_iter`, original collection is no longer accessible
    let names_to_iter = names;
    for name in names_to_iter.into_iter() {
        // Here the `name` is the actual value of the collection
        match name {
            "Ferris" => println!("A rustacean!"),
            _ => println!("Hello {}", name),
        }
    }

    // This will error `borrow of moved value`
    // println!("names: {:?}", names_to_iter);

    // This will error `you cant iter_mut for a immutable value`
    // for name in names.iter_mut() {}
    
    // Shadow
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "Rustacean!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
