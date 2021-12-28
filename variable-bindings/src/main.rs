fn main() {
    let immutable = 1; // Variables are immutable by default

    // use `mut` keyword to make it mutable
    let mut mutable = 2;

    // You can just open a block like this
    {
        let mut local = 3;
        mutable = 65;
    }
    // You can't acces local here
    // local = 4 throws an error

    // Remember immutable is immutable?
    // You can change that by redeclaring
    // Here this is called shadowing
    let mut immutable = 2;

    // When a new scope is started,
    // All the bindings such as immutabilities are frozen until the end of scope
    {
        // Shadowing immutable to immutable
        let immutable = 2;

        // This should throw an error
        // immutable = 5;
    }

    // But the binding is gone here
    immutable = 123;


}
