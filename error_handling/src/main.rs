use std::{num::ParseIntError, fmt::write, string::ParseError};


fn option_unwrap() {
    fn give_adult(drink: Option<&str>) {
        match drink {
            Some("lemonade") => println!("Yuck! Too sugary."),
            Some(inner) => println!("{}? How nice", inner),
            None => println!("No drink? Oh well."),
        }
    }
    
    fn drink(drink: Option<&str>) {
        let inside = drink.unwrap();
        if inside == "lemonade" {
            panic!("AAAaaaa!!!!");
        }
        println!("I love {}s!!!!!", inside);
    }

    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;
    drink(water);
    drink(coffee);
    drink(nothing);
}

fn unpacking_options_with() {
    struct Person {
        job: Option<Job>,
    }
    
    #[derive(Clone, Copy)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }
    
    #[derive(Clone, Copy)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: u32,
    }
    
    impl Person {

        // Gets the area code of the phone number of the person's job, if it exists.
        fn work_phone_area_code(&self) -> Option<u8> {
            // This would need many nested `match` statements without the `?` operator.
            // It would take a lot more code - try writing it yourself and see which
            // is easier.
            self.job?.phone_number?.area_code
        }
    }
    
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}

fn combinators() {
    #![allow(dead_code)]

    #[derive(Debug)] enum Food { Apple, Carrot, Potato }

    #[derive(Debug)] struct Peeled(Food);
    #[derive(Debug)] struct Chopped(Food);
    #[derive(Debug)] struct Cooked(Food);

    // Peeling food. If there isn't any, then return `None`.
    // Otherwise, return the peeled food.
    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None       => None,
        }
    }

    // Chopping food. If there isn't any, then return `None`.
    // Otherwise, return the chopped food.
    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            Some(Peeled(food)) => Some(Chopped(food)),
            None               => None,
        }
    }

    // Cooking food. Here, we showcase `map()` instead of `match` for case handling.
    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        chopped.map(|Chopped(food)| Cooked(food))
    }

    // A function to peel, chop, and cook food all in sequence.
    // We chain multiple uses of `map()` to simplify the code.
    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(|f| Peeled(f))
            .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
    }

    // Check whether there's food or not before trying to eat it!
    fn eat(food: Option<Cooked>) {
        match food {
            Some(food) => println!("Mmm. I love {:?}", food),
            None       => println!("Oh no! It wasn't edible."),
        }
    }
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // Let's try the simpler looking `process()` now.
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

fn some_different_ways() {
    #[derive(Debug)]
    enum Fruit {Apple, Orange, Banana, Kiwi, Lemon}
    
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;
    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}", first_available_fruit);

    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };
    
    let get_lemon_as_fallback2 = || {
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };
    
    let first_available_fruit = None.or_else(get_kiwi_as_fallback).or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}", first_available_fruit);


    let mut my_fruit: Option<Fruit> = None;
    let first_available_fruit = my_fruit.get_or_insert(Fruit::Apple);
    println!("first_avaailable_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);

    my_fruit = None;
    let second_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback2);
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback2);
    println!("second_available_fruit: {:?},   should_be_apple is: {:?}", second_available_fruit, should_be_apple);
}

fn result_hello() {
    fn multiply(first_number_str: &str, second_number_str: &str) ->i32 {
        let first_number = first_number_str.parse::<i32>().unwrap();
        let second_number = second_number_str.parse::<i32>().unwrap();
        first_number * second_number
    }

    let twenty = multiply("10", "2");
    println!("double is {twenty}");
    let error = multiply("tttt", "2");
    println!("double is {error}");
}

fn map_for_result() {
    use std::num::ParseIntError;

    fn multiply(first_number_str: &str, second_number_str: &str) ->Result<i32, ParseIntError> {
        match first_number_str.parse::<i32>() {
            Ok(first_number) => {
                match second_number_str.parse::<i32>() {
                    Ok(second_number) => {
                        Ok(first_number * second_number)
                    },
                    Err(e) => Err(e),
                }
            },
            Err(e) => Err(e),
        }
    }

    fn new_multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number*second_number)
        })
    }

    fn ha_multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = match first_number_str.parse::<i32>() {
            Ok(number) => number,
            Err(e) => return Err(e),
        };

        let second_number = match second_number_str.parse::<i32>() {
            Ok(number) => number,
            Err(e) => return Err(e),
        };

        Ok(first_number * second_number)
    }

    fn try_multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        Ok(first_number_str.parse::<i32>()? * second_number_str.parse::<i32>()?)
    }


    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(number) => println!("n is {number}"),
            Err(e) => println!("err is {e}"),
        }
    }

    let twenty = multiply("10", "2");
    print(twenty);
    let tt = new_multiply("10", "2");
    print(tt);
    let ha = ha_multiply("10", "2");
    print(ha);
    let try_h = try_multiply("10", "2");
    print(try_h);

}

fn aliases_for_result() {
    use std::num::ParseIntError;

    // Define a generic alias for a `Result` with the error type `ParseIntError`.
    type AliasedResult<T> = Result<T, ParseIntError>;

    // Use the above alias to refer to our specific `Result` type.
    fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
        })
    }

    // Here, the alias again allows us to save some space.
    fn print(result: AliasedResult<i32>) {
        match result {
            Ok(n)  => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}

fn pulling_result_out_of_options() {
    fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
        let opt = vec.first().map(|first| {
            first.parse::<i32>().map(|n| 2 * n)
        });

        opt.map_or(Ok(None), |r| r.map(Some))
    }

    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["abc", "93", "18"];
    
    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(empty));
    println!("The first doubled is {:?}", double_first(strings));
}

fn defining_error_type() {
    use std::fmt;

    type Result<T> = std::result::Result<T, DoubleError>;

    #[derive(Debug, Clone)]
    struct DoubleError;

    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first().ok_or(DoubleError).and_then(|s| {
            s.parse::<i32>().map_err(|_| DoubleError).map(|i| 2 * i)
        })
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {n}"),
            Err(e) => println!("Error: {e}"),
        }
    }

    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

fn boxing_errors() {
    use std::error;
    use std::fmt;

    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug, Clone)]
    struct EmptyVec;

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl error::Error for EmptyVec {}

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first()
            .ok_or_else(|| EmptyVec.into())
            .and_then(|s| {
                s.parse::<i32>()
                    .map_err(|e| e.into())
                    .map(|i| 2 * i)
            })
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {n}"),
            Err(e) => println!("Error: {e}"),
        }
    }

    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

fn other_use_of() {
    use std::error;
    use std::fmt;

    type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

    #[derive(Debug)]
    struct EmptyVec;

    impl fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "invalid first item ot double")
        }
    }

    impl error::Error for EmptyVec {}

    fn double_first(vec: Vec<&str>) ->Result<i32> {
        let first = vec.first().ok_or(EmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {n}"),
            Err(e) => println!("Error: {e}"),
        }
    }

    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}

fn wrapping_errors() {
    use std::error;
    use std::error::Error;
    use std::num::ParseIntError;
    use std::fmt;

    type Result<T> = std::result::Result<T, DoubleError>;

    #[derive(Debug)]
    enum DoubleError {
        EmptyVec,
        Parse(ParseIntError),
    }

    impl fmt::Display for DoubleError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
                DoubleError::Parse(..) => write!(f, "the provided string could not be parsed as int"),
            }
        }
    }

    impl error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match *self {
                DoubleError::EmptyVec => None,
                DoubleError::Parse(ref e) => Some(e),
            }
        }
    }

    impl From<ParseIntError> for DoubleError {
        fn from(err: ParseIntError) -> Self {
            DoubleError::Parse(err)
        }
    }
}

fn iterating_over_results() {
    
    fn first_case() {
        let strings = vec!["tofu", "93", "18"];
        let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
        println!("Results: {numbers:?}");
    }

    fn second_case() {
        let strings = vec!["tofu", "93", "18"];
        let numbers: Vec<_> = strings.into_iter().filter_map(|s| s.parse::<i32>().ok()).collect();
        println!("Results: {numbers:?}");
    }

    fn third_case() {
        let strings = vec!["42", "tofu", "93", "999", "18"];
        let mut errors = vec![];
        let numbers: Vec<_> = strings.into_iter()
            .map(|s| s.parse::<u8>())
            .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
            .collect();
        println!("Numbers: {numbers:?}");
        println!("Errors: {errors:?}");
    }

    first_case();
    second_case();
    third_case();
}

fn main() {
    // option_unwrap();
    // unpacking_options_with();
    // combinators();
    // some_different_ways();
    // result_hello();
    // map_for_result();
    // // aliases_for_result();
    // pulling_result_out_of_options();
    // defining_error_type();
    // boxing_errors();
    // other_use_of();
    // wrapping_errors();
    iterating_over_results();
}
