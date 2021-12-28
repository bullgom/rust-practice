use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum or a struct
impl List {
    // A constructor? Create an empty list
    fn new() -> List {
        Nil // Nil has type `List`
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type `List`
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(self) -> u32 {
        /*
         * `self` has to be matched, because the behavior of this method 
         * depends on the variant of `self`
         * `self` has type `&List`, and `*self` has type `List`, matching on a
         * concrete type `T` is preferred over a match on a reference `&T`
         */

        match self {
            // Can't take ownership of the tail, because `self` is borrowed
            // instead take a reference to the tail
            // We don't care about the element, so _
            Cons(_, tail) => 1+tail.len(),
            // Base case: An empty list has zezro length
            Nil => 0
        }
    }
}

fn main() {
    let mut list = List::new();

    // Add some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("Linked list has length: {}", list.len());

}
