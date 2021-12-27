fn main() {
    /*
     * Array is a collection of objects of the same type `T`, stored in contiguous
     * memory.
     * Type signature is [T; length]
     */
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

    println!("First element: {}", xs[0]);
    println!("Second element: {}", xs[1]);

    println!("Length of xs: {}", xs.len());

    // let ys_sliced = ys[1 .. 6]; will raise an `doesn't have a size known at compile-time error`

    // But you can borrow(By reference) a slice
    let ys_sliced = &ys[1 .. 6];

    println!("Sliced: {:?}", ys_sliced);
}
