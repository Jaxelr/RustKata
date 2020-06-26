fn main() {
    let amount = 20;
    fizz_buzz_if(amount);
    fizz_buzz_composed_if(amount);
    fizz_buzz_match(amount);
    fizz_buzz_enum(amount);

    for x in (1..amount).map(fizz_buzz_if_return) {
        println!("{}", x);
    }

    for x in (1..amount).map(fizz_buzz_match_return) {
        println!("{}", x);
    }

    for x in (1..amount)
    .map(|x| fizz_buzz_enum_return(x)) {
        println!("{}", x);
    }
}

fn fizz_buzz_if(amount: u32) -> () {
    for n in 1..=amount {
        let result = if n % 15 == 0 {
            "fizzbuzz".to_string()
        } else if n % 3 == 0 {
            "fizz".to_string()
        } else if n % 5 == 0 {
            "buzz".to_string()
        } else {
            n.to_string()
        };

        println!("{}", result);
    }
}

fn fizz_buzz_if_return(element: u32) -> String {   
    if element % 15 == 0 {
        "fizzbuzz".to_string()
    } else if element % 3 == 0 {
        "fizz".to_string()
    } else if element % 5 == 0 {
        "buzz".to_string()
    } else {
        element.to_string()
    }
}

fn fizz_buzz_composed_if(element: u32)
{
    if is_divisible_by(element, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(element, 3) {
        println!("fizz");
    } else if is_divisible_by(element, 5) {
        println!("buzz");
    } else {
        println!("{}", element);
    }
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }

    lhs % rhs == 0
}

fn fizz_buzz_match(amount: u32) -> () {
    for n in 1..=amount {
        let result = match (n % 3, n % 5) {
            (0, 0) => "fizzbuzz".to_string(), 
            (0, _) => "fizz".to_string(), 
            (_, 0) => "buzz".to_string(), 
            (_, _) => n.to_string(), 
        };

        println!("{}", result);
    }
}

fn fizz_buzz_match_return(element: u32) -> String {
    match (element % 3, element % 5) {
        (0, 0) => "fizzbuzz".to_string(), 
        (0, _) => "fizz".to_string(), 
        (_, 0) => "buzz".to_string(), 
        (_, _) => element.to_string(), 
    }
}

use std::fmt;

enum Counter {
    Fizz,
    Buzz,
    Fizzbuzz,
    Number(u32)
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Counter::Fizz => f.write_str("Fizz"),
            Counter::Buzz => f.write_str("Buzz"),
            Counter::Fizzbuzz => f.write_str("FizzBuzz"),
            Counter::Number(num) => write!(f, "{}", num),
        }
    }
}

fn fizz_buzz_enum_return(element: u32) -> Counter {
    match (element % 3, element % 5) {
        (0, 0) => Counter::Fizzbuzz, 
        (0, _) => Counter::Fizz, 
        (_, 0) => Counter::Buzz, 
        (_, _) => Counter::Number(element), 
    }
}

fn fizz_buzz_enum(amount: u32) -> () {
    for n in 1..=amount {
        let result = match (n % 3, n % 5) {
            (0, 0) => Counter::Fizzbuzz, 
            (0, _) => Counter::Fizz, 
            (_, 0) => Counter::Buzz, 
            (_, _) => Counter::Number(n), 
        };

        println!("{}", result);
    }
}