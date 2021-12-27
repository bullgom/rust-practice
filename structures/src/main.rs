use std::fmt;

/*
 * There are 3 types of structures
 * 1. C like Struct
 * 2. Tuple Struct or Named tuples
 * 3. Unit structs
 */

// C like struct
struct Person {
    name: String,
    age: u8,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.age)
    }
}

// Unit Struct
// Like an alias?
struct Unit;

// Tuple Struct
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;

    let x_diff = x1 - x2;
    let y_diff = y1 - y2;
    x_diff * y_diff
}

fn square(Point{x, y}: Point, length: f32) -> Rectangle {
    let rect = Rectangle{
        top_left: Point{x, y: y+length},
        bottom_right: Point{x: x+length, y}
    };

    rect
}

fn main() {
    let name = String::from("Maple");
    let age = 24;
    let maple = Person { name, age };

    println!("{}", maple);

    // Instantiate a Point
    let point: Point = Point { x: 5.3, y: 0.4 };
    // Access a field
    println!("Point 1 {}", point);

    // You can copy values of a struct
    let mut bottom_right = Point { x: 10.2, ..point };
    bottom_right.y = -0.55;

    println!("Point 2 {}", bottom_right);

    // You can destructure a struct
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    let unit = Unit;

    println!("Area of rectangle is {}", rect_area(rectangle));

    let rect = square(point, 5.);
    println!("Area of rectange is {}", rect_area(rect));
}
