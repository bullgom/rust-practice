fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.;
    let an_int = 4i32;

    /*
     * Variables in Rust are constant by default
     * `an_int = 1;`
     * will raise error
     * You need to use the `mut` keyword
     */

    let mut mutable_int = 4;
    mutable_int = 5;

    /*
     * You can't change a type of a variable
     * `mutable_int = false;`
     * But you can overwrite the variable signature
     * `let mut mutable_int = false;`
     * This is called `shadowing` a variable
     */

    let mut mutable_int = false;
    mutable_int = true;
}
