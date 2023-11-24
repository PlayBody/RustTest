use std::fmt;

#[allow(dead_code)]
fn learn_print() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "OK");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    //fmt::Display.
    // fmt::Display::fmt(&Structure(3), &mut ::std::io::stdout());
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi: f64 = 3.1415926535897932;
    println!("pi = {:#<10}", pi);
}

#[allow(dead_code)]
fn learn_debug() {
    // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
    // is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printable
    // also.
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
                "Slater",
                "Christian",
                actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter: Person<'_> = Person { name, age };

    // Pretty print
    println!("{:#?} {}, {}", peter, peter.name, peter.age);
}

#[allow(dead_code)]
fn learn_display() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl fmt::Binary for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({:b}, {:b})", self.x, self.y)
        }
    }

    let p1: Point = Point { x: 1, y: 2 };
    let p2: Point = Point { x: 3, y: 4 };

    println!("p1: {}, p2: {}", p1, p2);
    println!("p1: {:?}, p2: {:?}", p1, p2);
    println!("p1: {:b}, p2: {:b}", p1, p2);

    #[derive(Debug)]
    struct MinMax(i32, i32);
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }
    let minmax: MinMax = MinMax(1, 15);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    let big_range: MinMax = MinMax(-300, 300);
    let small_range: MinMax = MinMax(-100, 100);
    println!("The big range is {}, the small range is {}", big_range, small_range);

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
    let complex = Complex { real: 3.0, imag: 4.5 };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

}

#[allow(dead_code)]
fn learn_display_vec() {
    #[derive(Debug)]
    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec: &Vec<i32> = &self.0;
            write!(f, "[")?;
            for (i, v) in vec.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{i}:{v}")?;
            }
            write!(f, "]")
        }
    }
    let list: List = List(vec![1, 4, 8]);
    println!("Display: {}", list);
}

#[allow(dead_code)]
fn learn_add_two(a : i32, b : i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn learn_display_more() {
    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }
    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "RGB ({:0>3}, {:0>3}, {:0>3}) 0x{:0>6X}", self.red, self.green, self.blue, self.red as u32 * 0x10000 + self.green as u32 * 0x0100 + self.blue as u32)
        }
    }
    println!("Color: {}", Color { red: 0, green: 100, blue: 255 });
}

fn main() {
    // learn_print();
    // learn_debug();
    // learn_display();
    // learn_display_vec();
    // println!("{}", learn_add_two(4, 5));
    learn_display_more();
}
