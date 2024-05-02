use std::fmt;
use std::fmt::Display;

fn main() {
    // This is a line comment
    /*
     * This is a block comment
     */
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!("Base 10: {}", 69420);
    println!("Base 2: {:b}", 69420);
    println!("Base 8: {:o}", 69420);
    println!("Base 16: {:x}", 69420);
    println!("Base 16: {:X}", 69420);
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0>width$}", number = 1, width = 5);
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Structure(i32);
    impl Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    println!("This struct `{}` won't print...", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    #[derive(Debug)]
    #[allow(dead_code)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Deep(Structure);

    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);

    #[derive(Debug)]
    struct MinMax(i64, i64);
    impl Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }
    impl Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    impl Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", complex);
    println!("Debug: {}", complex);

    struct List(Vec<i32>);
    impl Display for List {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }
    impl Display for City {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
            write!(
                f,
                "{}: {:.3}`{} {:.3}`{}",
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
    impl Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue)
        }
    }

    for city in [
        City {
            name: "Dublin",
            lat: 53.829303,
            lon: -6.238292,
        },
        City {
            name: "Oslo",
            lat: 59.829303,
            lon: 10.238292,
        },
        City {
            name: "Vancuber",
            lat: 49.829303,
            lon: -123.238292,
        },
    ] {
        println!("{}", city);
    }
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        println!("{}", color);
    }
}
