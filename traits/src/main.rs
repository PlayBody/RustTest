use std::iter;
use std::vec::IntoIter;

fn hello() {
    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    trait Animal {
        fn new(name: &'static str) -> Self;
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }
        fn shear(&mut self) {
            if self.is_naked() {
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);
                self.naked = true;
            }
        }
    }

    impl Animal for Sheep {
        fn new(name: &'static str) -> Sheep {
            Sheep { naked: false, name: name }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaah?"
            } else {
                "baaah!"
            }
        }
        fn talk(&self) {
            // For example, we can add some quiet contemplation.
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }


    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

fn derive() {
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }
    
    struct Seconds(i32);


    let _one_second = Seconds(1);

    // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    //println!("One second looks like: {:?}", _one_second);
    // TODO ^ Try uncommenting this line

    // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
    //let _this_is_true = (_one_second == _one_second);
    // TODO ^ Try uncommenting this line

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}

fn return_dyn() {
    struct Sheep;
    struct Cow;

    trait Animal {
        fn noise(&self) -> &'static str;
    }

    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "baaaah!"
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "moooo!"
        }
    }

    fn random_animal(random_number: f64) -> Box<dyn Animal>{
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

fn operator_overloading() {
    use std::ops;

    struct Foo;
    struct Bar;

    #[derive(Debug)]
    struct FooBar;

    #[derive(Debug)]
    struct BarFoo;

    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            println!("> Foo.add(Bar) was called");
            FooBar
        }
    }

    impl ops::Add<Foo> for Bar {
        type Output = BarFoo;

        fn add(self, _rhs: Foo) -> BarFoo {
            println!("> Bar.add(Foo) was called");
            BarFoo
        }
    }
    println!("Foo + Bar = {:?}", Foo+Bar);
    println!("Foo + Bar = {:?}", Bar+Foo);
}

fn trait_drop() {
    struct Droppable {
        name: &'static str,
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }

    let _a = Droppable{name: "a"};
    {
        let _b = Droppable{name: "b"};
        {
            let _c = Droppable{name: "c"};
            let _d = Droppable{name: "d"};
            println!("Exiting block B");
        }
        println!("Just exited block B");
        println!("Exiting block A");
    }
    println!("Just exited block A");
    drop(_a);
    println!("End of drop function");
}

fn trait_iterator() {
    struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Iterator for Fibonacci {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            let current = self.curr;
            self.curr = self.next;
            self.next = current + self.next;
            Some(current)
        }
    }

    fn fibonacci() -> Fibonacci {
        Fibonacci {curr: 0, next: 1}
    }

    // `0..3` is an `Iterator` that generates: 0, 1, and 2.
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for` works through an `Iterator` until it returns `None`.
    // Each `Some` value is unwrapped and bound to a variable (here, `i`).
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms.
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms.
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}

fn trait_impl() {
    fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
        src.lines()
            .map(|line| {
                // For each line in the source
                line.map(|line| {
                    // If the line was read successfully, process it, if not, return the error
                    line.split(',') // Split the line separated by commas
                        .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                        .collect() // Collect all strings in a row into a Vec<String>
                })
            })
            .collect() // Collect all lines into a Vec<Vec<String>>
    }

    fn parse_csv_documents(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
        src.lines()
            .map(|line| {
                // For each line in the source
                line.map(|line| {
                    // If the line was read successfully, process it, if not, return the error
                    line.split(',') // Split the line separated by commas
                        .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                        .collect() // Collect all strings in a row into a Vec<String>
                })
            })
            .collect() // Collect all lines into a Vec<Vec<String>>
    }
    
    fn combine_vecs_explicit_return_type(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    fn combine_vecs(
        v: Vec<i32>,
        u: Vec<i32>,
    ) -> impl Iterator<Item=i32> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");



    fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
        let closure = move |x: i32| { x + y };
        closure
    }
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);


    fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
        numbers
            .iter()
            .filter(|x| x > &&0)
            .map(|x| x * 2)
    }
    
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}

fn trait_clone() {
    // A unit struct without resources
    #[derive(Debug, Clone, Copy)]
    struct Unit;

    // A tuple struct with resources that implements the `Clone` trait
    #[derive(Clone, Debug)]
    struct Pair(Box<i32>, Box<i32>);

    // Instantiate `Unit`
    let unit = Unit;
    // Copy `Unit`, there are no resources to move
    let copied_unit = unit;

    // Both `Unit`s can be used independently
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    // Instantiate `Pair`
    let pair: Pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    // Error! `pair` has lost its resources
    //println!("original: {:?}", pair);
    // TODO ^ Try uncommenting this line

    // Clone `moved_pair` into `cloned_pair` (resources are included)
    let cloned_pair = moved_pair.clone();
    // Drop the original pair using std::mem::drop
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    //println!("copy: {:?}", moved_pair);
    // TODO ^ Try uncommenting this line

    // The result from .clone() can still be used!
    println!("clone: {:?}", cloned_pair);
}

fn supertraits() {
    trait Person {
        fn name(&self) -> String;
    }
    
    // Person is a supertrait of Student.
    // Implementing Student requires you to also impl Person.
    trait Student: Person {
        fn university(&self) -> String;
    }
    
    trait Programmer: Person {
        fn fav_language(&self) -> String;
    }
    
    // CompSciStudent (computer science student) is a subtrait of both Programmer 
    // and Student. Implementing CompSciStudent requires you to impl both supertraits.
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }
    
    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }
}

fn disambiguating_overlaping_traits() {
    trait UsernameWidget {
        fn get(&self) -> String;
    }

    trait AgeWidget {
        fn get(&self) -> u8;
    }

    struct Form {
        username: String,
        age: u8,
    }

    impl UsernameWidget for Form {
        fn get(&self) -> String {
            self.username.clone()
        }
    }

    impl AgeWidget for Form {
        fn get(&self) -> u8 {
            self.age
        }
    }

    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    let username = <dyn UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <dyn AgeWidget>::get(&form);
    assert_eq!(28, age);
}

fn main() {
    hello();
    derive();
    return_dyn();
    operator_overloading();
    trait_drop();
    trait_iterator();
    trait_impl();
    trait_clone();
    supertraits();
    disambiguating_overlaping_traits();
}
