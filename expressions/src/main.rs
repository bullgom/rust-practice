
// A program is mostly made up of series of statements
/*
 * fn main() {
 *     // statement
 *     // statement
 *     ...
 * }
 */

/*
 * There are multiple kinds of statements in Rust
 * Most common: Variable, Expression with`;`
 * 
 * Blocks can be expressions also!
 * 
 */
fn main() {
    
    let x = 5u32;

    // Recall that Blocks are also expressions
    // So it can be used for assignments
    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;

        x_cubed + x_squared + x
    }; // You need the `;` for the Block to be seen as an expression
}
