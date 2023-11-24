use std::fmt;
use std::mem;

fn basic_operations() {
    // Variables can be type annotated.
    let _logical: bool = true;

    let _a_float: f64 = 1.0;  // Regular annotation
    let _an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let _default_float   = 3.0; // `f64`
    let _default_integer = 7;   // `i32`

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    #[allow(unused_assignments)]
    let _mutable = true;
}

fn literals_operators() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

fn tuples() {
    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
        }
    }

    fn reverse(matrix: Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }

    let matrix: Matrix = Matrix(1.0, 2.0, 3.0, 4.0);
    println!("Transpose:\n{}", reverse(matrix));
}

fn arrays_slices() {
    let a: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let ys: [[i32; 10]; 500] = [[0; 10]; 500];
    let slice = &a[2..5];
    println!("Slice: {:?}", slice);
    let slice = &a[..5];
    println!("Slice: {:?}", slice);
    let slice = &a[2..];
    println!("Slice: {:?}", slice);
    println!("Slice Size: {} {} {}", ys.len(), ys[0].len(), mem::size_of_val(&ys));

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    for i in 0..a.len() + 1 { // Oops, one element too far!
        match a.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
    println!("{:?}", Some(&7));
    assert_eq!(Some(&2), a.get(1));
    // Out of bound indexing on array causes compile time error.
    println!("{}", a[9]);
    // Out of bound indexing on slice causes runtime error.
    println!("{}", a[..][9]);
}

fn main() {
    basic_operations();
    literals_operators();
    tuples();
    arrays_slices();
}
