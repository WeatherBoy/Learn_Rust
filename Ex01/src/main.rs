use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

/*
    Exercise 1.2.2:
        Creating a Complex structure which can be printed in a certain way!
*/
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // customized such that it is only shown as a complex number
        write!(f, "{} + {}i", self.real, self.imag)
    }
}
// ************************************************************************************************

/*
    Exercise 1.2.2.1
        Something about implementing Display for a list.. :O
*/

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?; // <-- Implemented it such that it also shows the index
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}
// ************************************************************************************************

/*
    Exercise 1.2.3
        Something about making a Color struct displayable
*/

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
}

impl fmt::Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// The MEAT of ex. 1.2.3 ########################
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Very much only a good implementation if you are deep into *rust* formating.
        // The format reads as {n : s<>e f}:
        //      n: the n'th parameter I want (parsed as the third and onwards parameter to *write!*)
        //      s: what I want to substiture with.
        //      <>: relational operater (less than or grater than)-
        //      f: the format I want it in.
        //
        // So in this case {1 : 0>2 X}:
        //      1: I want the second parameter after this string.
        //      0: I want to substitute (pad) with zeroes
        //      2: Up to two spaces
        //      X: I want it formatted as hexadecimal (captital letters).
        write!(
            f,
            "RGB ({0}, {1}, {2}) 0x{0:0>2X}{1:0>2X}{2:0>2X}",
            self.red, self.green, self.blue
        )
    }
}
// ************************************************************************************************<

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);

    /*
        Testing whether exercise 1.2.2 was implemented correctly!
    */
    let complex_num = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("\n\n*** Ex 1.2.2 ***");
    println!("Display: {}", complex_num);
    println!("Debug: {:?}", complex_num);

    /*
        Testing whether exercise 1.2.2.1 was implemented correctly!
    */
    println!("\n\n*** Ex 1.2.2.1 ***");
    let v = List(vec![1, 2, 3, 4, 5]);
    println!("{}", v);

    /*
        Testing whether exercise 1.2.3 was implemented correctly!
    */
    println!("\n\n*** Ex 1.2.3 ***");
    // Cities ###################################
    let dublin = City {
        name: "Dublin",
        lat: 53.347778,
        lon: -6.259722,
    };
    let oslo = City {
        name: "Oslo",
        lat: 59.95,
        lon: 10.75,
    };
    let vancouver = City {
        name: "Vancouver",
        lat: 49.25,
        lon: -123.1,
    };

    // Colors ###################################
    let color01 = Color {
        red: 128,
        green: 255,
        blue: 90,
    };
    let color02 = Color {
        red: 0,
        green: 3,
        blue: 254,
    };
    let color03 = Color {
        red: 0,
        green: 0,
        blue: 0,
    };

    // Loops ####################################
    for city in [dublin, oslo, vancouver].iter() {
        println!("{}", *city);
    }

    for color in [color01, color02, color03].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", *color);
    }
}
