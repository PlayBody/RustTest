fn hello() {
    macro_rules! say_hello {
        // `()` indicates that the macro takes no argument.
        () => {
            // The macro will expand into the contents of this block.
            println!("Hello!")
        };
    }

    say_hello!();
}

fn designators() {
    macro_rules! create_function {
        // This macro takes an argument of designator `ident` and
        // creates a function named `$func_name`.
        // The `ident` designator is used for variable/function names.
        ($func_name:ident) => {
            fn $func_name() {
                // The `stringify!` macro converts an `ident` into a string.
                println!("You called {:?}()",
                         stringify!($func_name));
            }
        };
    }
    
    // Create functions named `foo` and `bar` with the above macro.
    create_function!(foo);
    create_function!(bar);
    
    macro_rules! print_result {
        // This macro takes an expression of type `expr` and prints
        // it as a string along with its result.
        // The `expr` designator is used for expressions.
        ($expression:expr) => {
            // `stringify!` will convert the expression *as it is* into a string.
            println!("{:?} = {:?}",
                     stringify!($expression),
                     $expression);
        };
    }

    foo();
    bar();

    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}

fn overload() {
    macro_rules! test {
        ($left:expr; and $right:expr) => {
            println!("{:?} and {:?} is {:?}", stringify!($left), stringify!($right), $left && $right);
        };
        ($left:expr; or $right:expr) => {
            println!("{:?} or {:?} is {:?}", stringify!($left), stringify!($right), $left || $right);
        };
    }
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}

fn repeat() {
    macro_rules! find_min {
        ($x:expr) => ($x);
        ($x:expr, $($y:expr),+) => {
            std::cmp::min($x, find_min!($($y),+))
        };
    }
    println!("{}", find_min!(1));
    println!("{}", find_min!(1+2, 2));
    println!("{}", find_min!(5, 2*3, 4));
}

fn dry_code() {
    use std::ops::{Add, Mul, Sub};

    macro_rules! assert_equal_len {
        ($a:expr, $b:expr, $func:ident, $op:tt) => {
            assert!($a.len() == $b.len(), "{:?}: dimension mismatch: {:?} {:?} {:?}", stringify!($func), ($a.len(),), stringify!($op), ($b.len(),));
        };
    }

    macro_rules! op {
        ($func:ident, $bound:ident, $op:tt, $method:ident) => {
            fn $func<T: $bound<T, Output=T> + Copy> (xs: &mut Vec<T>, ys: &Vec<T>) {
                assert_equal_len!(xs, ys, $func, $op);

                for (x, y) in xs.iter_mut().zip(ys.iter()) {
                    *x = $bound::$method(*x, *y);
                }
            }
        };
    }

    op!(add_assign, Add, +=, add);
    op!(mul_assign, Mul, *=, mul);
    op!(sub_assign, Sub, -=, sub);
    // mod test {
    //     use std::iter;

    //     macro_rules! test {
    //         ($func:ident, $x:expr, $y:expr, $z:expr) => {
    //             #[test]
    //             fn $func() {
    //                 for size in 0usize..10 {
    //                     let mut x: Vec<_> = iter::repeat($x).take(size).collect();
    //                     let y: Vec<_> = iter::repeat($y).take(size).collect();
    //                     let z: Vec<_> = iter::repeat($z).take(size).collect();
    //                 }
    //                 super::$func(&mut x, &y);

    //                 assert_eq!(x, z);
    //             }
    //         };
    //     }
        
    //     test!(add_assign, 1u32, 2u32, 3u32);
    //     test!(mul_assign, 2u32, 3u32, 6u32);
    //     test!(sub_assign, 3u32, 2u32, 1u32);
    // }

}

fn dsl_code() {
    macro_rules! calculate {
        (eval $e:expr) => {
            {
                let val: usize = $e;
                println!("{} = {}", stringify!($e), val);
            }
        };
        (eval $e:expr, $(eval $es:expr),+) => {
            {
                calculate!{eval $e}
                calculate!{$(eval $es), +}
            }
        }
    }
    calculate!{
        eval 3*0
    }
    calculate!(eval 5+5);

    calculate! { // Look ma! Variadic `calculate!`!
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}

fn main() {
    hello();
    designators();
    overload();
    repeat();
    dry_code();
    dsl_code();
}
