#![allow(dead_code)]

use std::fmt;

fn structures() {

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    
    struct Unit;
    
    struct Pair(i32, f32);
    
    struct Point {
        x: f32,
        y: f32,
    }

    let point = Point { x: 1.1, y: 2.3 };
    
    struct Rectangle {
        top_left: Point,
        right_bottom: Point,
    }

    let name: String = String::from("John");
    let age: u8 = 30;
    
    let Point { x: left_edge, y: top_edge } = point;
    println!("{} {} === {} {}", left_edge, top_edge, name, age);
}

fn enums() {
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click{x:i64, y:i64},
    }

    fn inspect(event: &WebEvent) {
        match event {
            WebEvent::PageLoad => println!("Page loaded"),
            WebEvent::PageUnload => println!("Page unloaded"),
            WebEvent::KeyPress(key) => println!("Key pressed: {}", key),
            WebEvent::Paste(text) => println!("Pasted text: {}", text),
            WebEvent::Click{x, y} => println!("Clicked at ({}, {})", x, y),
        }
    }
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(&pressed);
    inspect(&pasted);
    inspect(&click);
    inspect(&load);
    inspect(&unload);
}

fn enums_impls() {
    enum TodoWithNumbers {
        Add,
        Subtract,
    }
    impl TodoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                TodoWithNumbers::Add => x + y,
                TodoWithNumbers::Subtract => x - y,
            }
        }
    }
    let x: i32 = 10;
    let y: i32 = 5;
    let add = TodoWithNumbers::Add;
    let subtract = TodoWithNumbers::Subtract;
    println!("{}, {}", add.run(x, y), subtract.run(x, y));
}

fn enums_cast() {
    enum TodoWithNumbers {
        Add,
        Subtract,
    }
    let add = TodoWithNumbers::Add;
    let subtract = TodoWithNumbers::Subtract;
    println!("{}, {:06X}", add as i32, subtract as i32);
}

fn enums_test_linked_list() {
    enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List {
            List::Nil
        }

        fn prepend(self, value: u32) -> List {
            List::Cons(value, Box::new(self))
        }
        fn append(self, value: u32) -> List {
            match self {
                List::Nil => List::Cons(value, Box::new(List::Nil)),
                List::Cons(head, tail) => {
                    List::Cons(head, Box::new(tail.append(value)))
                }
            }
        }
        fn len(&self) -> u32 {
            match self {
                List::Nil => 0,
                List::Cons(_, tail) => 1 + tail.len(),
            }
        }
        fn stringify(&self) -> String {
            let mut current: &List = self;
            let mut s: String = String::from("[");
            let mut first = true;
            while let List::Cons(head, next) = current {
                if !first {
                    s += ", ";
                } else {
                    first = false;
                }
                s += &format!("{}", head);
                current = next;
            }
            s += "]";
            s
        }
    }
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.append(5);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

fn constants() {
    const PI:f64 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679;
    println!("PI is {}", PI);

    fn is_big(n: i32) -> bool {
        // Access constant in some function
        n > THRESHOLD
    }

    let n = 16;
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    static LANGUAGE: &str = "Rust";
    const THRESHOLD: i32 = 10;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}

fn variable_freezing() {
    let mut _mutable_integer = 7i32;
    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer: i32 = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
        println!("{}", _mutable_integer);
        // FIXME ^ Comment out this line

        // `_mutable_integer` goes out of scope
    }
    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
    println!("{}", _mutable_integer);
}

// Suppress all warnings from casts which overflow.
#[allow(overflowing_literals)]

fn casting() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules.
    // A float cannot be directly converted to a char.
    // let character = decimal as char;
    // FIXME ^ Comment out this line

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);

    // In boundary case 128 value in 8-bit two's complement representation is -128
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the value of 232 in 8-bit two's complement representation is -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

fn literals() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn inference() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}

fn aliasing() {
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;
    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanoseconds: NanoSecond = 5 as NanoSecond;
    let inches: Inch = 2 as U64;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
                nanoseconds,
                inches,
                nanoseconds + inches);
}

fn from_into() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    // impl Into<Number> for i32 {
    //     fn into(self) -> Number {
    //         Number { value: self }
    //     }
    // }
    
    let num = Number::from(30);
    println!("My number is: {}", num.value);

    let int = 5;
    // Try removing the type annotation
    let num: Number = int.into();
    println!("My number is: {}", num.value);
}

fn try_from_try_into() {
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

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

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    println!("{:?}", result);
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
    println!("{:?}", result);
}

fn to_from_string() {
    // Converting to String
    #[derive(Debug)]
    struct Circle{
        radius: f64,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius: {}", self.radius)
        }
    }

    let circle: Circle = Circle { radius: 100.0 };
    println!("{:?} - {} - {}", circle, circle, circle.to_string());
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

fn main() {
    // structures();
    // enums();
    // enums_impls();
    // enums_cast();
    // enums_test_linked_list();
    // constants();
    // variable_freezing();
    // casting();
    // literals();
    // inference();
    // aliasing();
    // from_into();
    try_from_try_into();
    to_from_string();
}
