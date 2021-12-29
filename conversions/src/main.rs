// Remember that only some of types have conversions available?
// You can implement that with `from`

use std::convert::{From, TryFrom, TryInto}; // its implementation come from this

struct Number {
    value: i32,
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

/*
 * `From` is a trait
 * This trait defines how to create `Self` from another type
 * The library contains some impl of this trait for primitive and common types
 */
impl From<i32> for Number {

    // This is implementation for
    // `from` function for `Number`
    // for type `i32` to `Self`
    // Where `Self` (note that first letter is upper case, this means the class)
    fn from(item: i32) -> Self{
        Number {value: item}
    }
}

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    let integer = 30i32;
    let num = Number::from(integer);

    // `into` is a method implemented by default
    // it looks for the target type, and converts its value `into` the target type
    let another_num : Number = integer.into();
    // Note that it must be able to infer the target type
    // The following will raise an error and suggest you specify the target type
    // let another_num = integer.into();
    
    // For cases when type conversions may fail, use `try_from`, `try_into`
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
}
