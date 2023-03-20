// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// For display and such
use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

/*
    Exercise 3.1:
        Add a function rect_area which calculates the area of
        a Rectangle (try using nested destructuring)
*/

fn rect_area(rect: Rectangle) -> f32 {
    // I don't know whether this is the most elegant implemenation :/
    // But it is for sure one implementation...
    let Point { x: x1, y: y1 } = rect.bottom_right;
    let Point { x: x2, y: y2 } = rect.top_left;
    let delta_x: f32 = (x1 - x2).abs();
    let delta_y: f32 = (y1 - y2).abs();

    delta_x * delta_y
}

/*
    Second part:
        Add a function square which takes a Point and a f32 as arguments,
        and returns a Rectangle with its top left corner on the point,
        and a width and height corresponding to the f32.
*/
fn square(coord: Point, side: f32) -> Rectangle {
    let bottom_right = Point {
        x: coord.x + side,
        y: coord.y - side,
    };
    Rectangle {
        top_left: coord,
        bottom_right,
    }
}

/*
    Just because I can..
    (and because it was probably easier)
*/
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Point { x: x1, y: y1 } = self.bottom_right;
        let Point { x: x2, y: y2 } = self.top_left;
        write!(
            f,
            "Top left corner: ( x2: {}, y2: {} ) \nBottom right corner: ( x1: {}, y1: {} )",
            x2, y2, x1, y1
        )
    }
}
// ************************************************************************************************

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    /*
        Testing Ex. 3.1
    */

    let test_rect = Rectangle {
        top_left: Point { x: 6.9, y: 4.2 },
        bottom_right: Point { x: 30.0, y: 10.0 },
    };
    println!("\n\n*** Ex 3.1 ***");
    println!("Area of rectangle: {}", rect_area(test_rect));

    let coord = Point { x: 10.0, y: 20.0 };
    let pretty_square = square(coord, 5.0);
    println!("\nThe square: \n{}", pretty_square);
    println!("Area of the square: {}", rect_area(pretty_square));
}
