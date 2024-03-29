

fn function() {
    #[derive(Debug)]
    struct A;          // Concrete type `A`.
    struct S(A);       // Concrete type `S`.
    struct SGen<T>(T); // Generic type `SGen`.

    // The following functions all take ownership of the variable passed into
    // them and immediately go out of scope, freeing the variable.

    // Define a function `reg_fn` that takes an argument `_s` of type `S`.
    // This has no `<T>` so this is not a generic function.
    fn reg_fn(_s: S) {}

    // Define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`.
    // It has been explicitly given the type parameter `A`, but because `A` has not 
    // been specified as a generic type parameter for `gen_spec_t`, it is not generic.
    fn gen_spec_t(_s: SGen<A>) {}

    // Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`.
    // It has been explicitly given the type parameter `i32`, which is a specific type.
    // Because `i32` is not a generic type, this function is also not generic.
    fn gen_spec_i32(_s: SGen<i32>) {}

    // Define a function `generic` that takes an argument `_s` of type `SGen<T>`.
    // Because `SGen<T>` is preceded by `<T>`, this function is generic over `T`.
    fn generic<T>(_s: SGen<T>) {}

    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));

    let a = A;
    println!("{a:?}");
}

fn implementation() {
    struct Val {
        val: f64,
    }
    
    struct GenVal<T> {
        gen_val: T,
    }
    
    // impl of Val
    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }
    
    // impl of GenVal for a generic type `T`
    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }
    let x = Val {
        val: 3.0
    };
    let y = GenVal {
        gen_val: 1i32
    };
    println!("{} {}", x.value(), y.value());
}

fn traits() {
    // Non-copyable types.
    struct Empty;
    struct Null;

    // A trait generic over `T`.
    trait DoubleDrop<T> {
        // Define a method on the caller type which takes an additional single parameter `T` and does nothing with it.
        fn double_drop(self, _: T);
    }

    // Implement `DoubleDrop<T>` for any generic parameter `T` and
    // caller `U`.
    impl<T, U> DoubleDrop<T> for U {
        // This method takes ownership of both passed arguments, deallocating both.
        fn double_drop(self, _: T) {}
    }

    let empty = Empty;
    let null  = Null;

    // Deallocate `empty` and `null`.
    empty.double_drop(null);

    // empty.double_drop(null);
    // null;
    // ^ TODO: Try uncommenting these lines.
}

fn bounds() {
    use std::fmt::Debug;

    trait HasArea {
        fn area(&self) -> f64;
    }

    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }
    #[derive(Debug)]
    struct Trangle {
        a: f64,
        b: f64,
        c: f64,
    }
    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    impl HasArea for Trangle {
        fn area(&self) -> f64 {
            let k = (self.a + self.b + self.c) / 2.0;
            f64::sqrt(k*(k-self.a)*(k-self.b)*(k-self.c))
        }
    }

    let rect = Rectangle{
        width: 30f64,
        height: 30f64
    };
    let trangle = Trangle{
        a: 4f64,
        b: 5f64,
        c: 6f64,
    };

    fn get_area<T : HasArea>(t: &T) -> f64 {
        t.area()
    }

    println!("Rectangle debug:{:?} area:{}", rect, get_area(&rect));
    println!("Trangle debug:{:?} area:{}", trangle, get_area(&trangle));
}

fn empty_bounds() {
    struct Cardinal;
    struct BlueJay;
    struct Turkey;

    trait Red {}
    trait Blue {}

    impl Red for Cardinal {}
    impl Blue for BlueJay {}

    // These functions are only valid for types which implement these
    // traits. The fact that the traits are empty is irrelevant.
    fn red<T: Red>(_: &T)   -> &'static str { "red" }
    fn blue<T: Blue>(_: &T) -> &'static str { "blue" }
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ TODO: Try uncommenting this line.
}

fn multiple_bounds() {
    use std::fmt::{Debug, Display};

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}`", t);
        println!("u: `{:?}`", u);
    }
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    // TODO ^ Try uncommenting this.

    compare_types(&array, &vec);
}

fn where_clauses() {
    use std::fmt::Debug;
    trait PrintInOption {
        fn print_in_option(self);
    }
    
    // Because we would otherwise have to express this as `T: Debug` or 
    // use another method of indirect approach, this requires a `where` clause:
    impl<T> PrintInOption for T where
        Option<T>: Debug {
        // We want `Option<T>: Debug` as our bound because that is what's
        // being printed. Doing otherwise would be using the wrong bound.
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }
    let vec = vec![1, 2, 3];

    vec.print_in_option()
}

fn new_type_idom() {
    struct Years(i64);
    struct Days(i64);
    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }
    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }
    fn old_enought(age: &Years) -> bool {
        return age.0 >= 18
    }
    let years = Years(2);
    let days = years.to_days();
    println!("Old enougth {}", old_enought(&years));
    println!("Old enougth {}", old_enought(&days.to_years()));
    // println!("Old enougth {}", old_enought(&days));
}

fn associated_items() {
    struct Container(i32, i32);

    // A trait which checks if 2 items are stored inside of container.
    // Also retrieves first or last value.
    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requires `A` and `B`.
        fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`.
        fn last(&self) -> i32;  // Doesn't explicitly require `A` or `B`.
    }

    impl Contains<i32, i32> for Container {
        // True if the numbers stored are equal.
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        // Grab the first number.
        fn first(&self) -> i32 { self.0 }

        // Grab the last number.
        fn last(&self) -> i32 { self.1 }
    }

    // `C` contains `A` and `B`. In light of that, having to express `A` and
    // `B` again is a nuisance.
    fn difference<A, B, C>(container: &C) -> i32 where
        C: Contains<A, B> {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));

}

fn associated_types() {
    struct Container(i32, i32);

    // A trait which checks if 2 items are stored inside of container.
    // Also retrieves first or last value.
    trait Contains {
        // Define generic types here which methods will be able to utilize.
        type A;
        type B;
    
        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }
    
    impl Contains for Container {
        // Specify what types `A` and `B` are. If the `input` type
        // is `Container(i32, i32)`, the `output` types are determined
        // as `i32` and `i32`.
        type A = i32;
        type B = i32;
    
        // `&Self::A` and `&Self::B` are also valid here.
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }
        // Grab the first number.
        fn first(&self) -> i32 { self.0 }
    
        // Grab the last number.
        fn last(&self) -> i32 { self.1 }
    }
    
    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }
    

    let number_1 = 3;
    let number_2 = 10;

    let container: Container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}

fn main() {
    function();
    implementation();
    traits();
    bounds();
    empty_bounds();
    multiple_bounds();
    where_clauses();
    new_type_idom();
    associated_items();
    associated_types();
}
