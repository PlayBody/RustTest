use std::fmt::Debug;

fn raii() {
    fn create_box() {
        let _box1 = Box::new(3i32);
    }
    
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
}

fn drop_test() {
    struct ToDrop;

    impl Drop for ToDrop {
        fn drop(&mut self) {
            println!("ToDrop is being dropped");
        }
    }
    
    let _x = ToDrop;
    println!("Made a ToDrop!");
}

fn ownership_moves() {
    fn mutability() {
        let immutable_box = Box::new(5u32);

        println!("immutable_box contains {}", immutable_box);

        // Mutability error
        //*immutable_box = 4;

        // *Move* the box, changing the ownership (and mutability)
        let mut mutable_box = immutable_box;

        println!("mutable_box contains {}", mutable_box);

        // Modify the contents of the box
        *mutable_box = 4;

        println!("mutable_box now contains {}", mutable_box);
    }

    fn partial_moves() {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: Box<u8>,
        }

        let person = Person {
            name: String::from("Alice"),
            age: Box::new(20),
        };

        // `name` is moved out of person, but `age` is referenced
        let Person { name, ref age } = person;

        println!("The person's age is {}", age);

        println!("The person's name is {}", name);

        // Error! borrow of partially moved value: `person` partial move occurs
        // println!("The person struct is {:?}", person);

        // `person` cannot be used but `person.age` can be used as it is not moved

        println!("The person's age from person struct is {}", person.age);
    }

    // This function takes ownership of the heap allocated memory
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);

        // `c` is destroyed and the memory freed
    }
    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.
    
    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    // println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    //println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line


    mutability();
    partial_moves();

}


fn borrowing_mutability() {
    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    struct Book {
        // `&'static str` is a reference to a string allocated in read only memory
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    // This function takes a reference to a book
    fn borrow_book(book: &Book) {
        println!("I immutably borrowed {} - {} edition", book.title, book.year);
    }

    // This function takes a reference to a mutable book and changes `year` to 2014
    fn new_edition(book: &mut Book) {
        book.year = 2014;
        println!("I mutably borrowed {} - {} edition", book.title, book.year);
    }

    // Create an immutable Book named `immutabook`
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // Create a mutable copy of `immutabook` and call it `mutabook`
    let mut mutabook = immutabook;
    
    // Immutably borrow an immutable object
    borrow_book(&immutabook);

    // Immutably borrow a mutable object
    borrow_book(&mutabook);
    
    // Borrow a mutable object as mutable
    new_edition(&mut mutabook);
    
    // Error! Cannot borrow an immutable object as mutable
    // new_edition(&mut immutabook);
    // FIXME ^ Comment out this line
}

fn borrowing_aliasing() {
    struct Point { x: i32, y: i32, z: i32 }

    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Data can be accessed via the references and the original owner
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // Error! Can't borrow `point` as mutable because it's currently
    // borrowed as immutable.
    // let mutable_borrow = &mut point;
    // TODO ^ Try uncommenting this line

    // The borrowed values are used again here
    println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

    // The immutable references are no longer used for the rest of the code so
    // it is possible to reborrow with a mutable reference.
    let mutable_borrow = &mut point;

    // Change data via mutable reference
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Error! Can't borrow `point` as immutable because it's currently
    // borrowed as mutable.
    // let y = &point.y;
    // TODO ^ Try uncommenting this line

    // Error! Can't print because `println!` takes an immutable reference.
    // println!("Point Z coordinate is {}", point.z);
    // TODO ^ Try uncommenting this line

    // Ok! Mutable references can be passed as immutable to `println!`
    println!("Point has coordinates: ({}, {}, {})",
                mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // The mutable reference is no longer used for the rest of the code so it
    // is possible to reborrow
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}

fn borrowing_ref_pattern() {
    #[derive(Clone, Copy)]
    struct Point { x: i32, y: i32 }

    let c = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to
    // an `&` borrow on the right side.
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` is also valid when destructuring a struct.
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`.
        let Point { x: ref ref_to_x, y: _ } = point;

        // Return a copy of the `x` field of `point`.
        *ref_to_x
    };

    // A mutable copy of `point`
    let mut mutable_point = point;

    {
        // `ref` can be paired with `mut` to take mutable references.
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // Mutate the `y` field of `mutable_point` via a mutable reference.
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // A mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    
    {
        // Destructure `mutable_tuple` to change the value of `last`.
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    
    println!("tuple is {:?}", mutable_tuple);
}

fn lifetime_explicit_annotation() {
    fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("x is {x} and y is {y}");
    }

    fn failed_borrow<'a>() {
        let _x = 12i32;
        // let y: &'a i32 = &_x;
    }
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);
    failed_borrow();
}

fn lifetime_functions() {
    fn print_one<'a>(x: &'a i32) {
        println!("'print one': x is {x}");
    }

    fn add_one<'a>(x: &'a mut i32) {
        *x += 1;
    }

    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("'print multi': x is {x}, y is {y}");
    }

    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }
    let x = 7;
    let y = 9;
    print_one(&x);
    print_multi(&x, &y);
    let z = pass_x(&x, &y);
    print_one(z);
    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

fn lifetime_methods() {
    struct Owner(i32);

    impl Owner {
        fn add_one<'a>(&'a mut self) {
            self.0 += 1;
        }
        fn print<'x>(&'x self) {
            println!("'print': {}", self.0);
        }
    }

    let mut owner = Owner(18);
    owner.add_one();
    owner.print();
}

fn lifetime_structures() {
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }
    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed{x:&x, y:&y};
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {single:?}");
    println!("x and y are borrowed in {double:?}");
    println!("x is borrowed in {reference:?}");
    println!("y is *not* borrowed in {number:?}");
}

fn lifetime_traits() {
    #[derive(Debug)]
    struct Borrowed<'a> {
        x: &'a i32,
    }

    impl<'a> Default for Borrowed<'a> {
        fn default() -> Self {
            Self { x: &10 }
        }
    }

    let b: Borrowed = Default::default();
    println!("b is {b:?}");

    let a = Box::new(5);
    let b = &a;
    let c = *a;
    println!("b is {b} and c is {c}");
}

fn lifetime_bounds() {
    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);

    fn print<T>(t: T) where T: Debug {
        println!("'print': t is {:?}", t);
    }

    fn print_ref<'a, T>(t: &'a T) where T: Debug + 'a {
        println!("'print_ref': t is {:?}", t);
    }

    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);
}

fn lifetime_coercion() {
    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }
    fn choose_first<'a:'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
        first
    }

    let first = 2;

    {
        let second = 3;

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
    println!("{}", first);
}

fn lifetime_static() {
    static NUM: i32 = 18;
    fn coerce_static<'a, 'b>(_: &'a i32, _: &'b i32) -> &'b i32 {
        &NUM
    }

    let a = {
        let static_string = "I am in read-only memory";
        println!("static_string: {}", static_string);
        static_string
    };
    let coerced_static: &i32;
    let aaa: &i32 = &5;
    println!("Return number {a}");
    {
        let lifetime_num = 9;
        coerced_static = coerce_static(&lifetime_num, aaa);
        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {NUM} stays accessible!");
    println!("coer: {}", coerced_static);


    fn random_vec() -> &'static [usize; 100] {    
        let boxed = Box::new([0; 100]);
        Box::leak(boxed)
    }
    let first: &'static [usize; 100] = random_vec();
    let second: &'static [usize; 100] = random_vec();
    assert_ne!(first, second);

    fn print_it( input: impl Debug+'static ) {
        println!( "'static value passed in is: {:?}", input );
    }
    
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    // print_it(&i);

}

fn lifetime_elision() {
    fn elided_input(x: &i32) {
        println!("`elided_input`: {}", x);
    }
    
    fn annotated_input<'a>(x: &'a i32) {
        println!("`annotated_input`: {}", x);
    }
    
    fn elided_pass(x: &i32) -> &i32 { x }

    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

    let x = 3;

    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));
}

fn main() {
    raii();
    drop_test();
    ownership_moves();
    borrowing_mutability();
    borrowing_aliasing();
    borrowing_ref_pattern();
    lifetime_explicit_annotation();
    lifetime_functions();
    lifetime_methods();
    lifetime_structures();
    lifetime_traits();
    lifetime_bounds();
    lifetime_coercion();
    lifetime_static();
    lifetime_elision();
}
