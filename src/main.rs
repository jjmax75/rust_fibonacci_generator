use std::io;

fn main() {
    println!("Which Fibonacci number would you like?");

    let mut nth = String::new();

    io::stdin()
        .read_line(&mut nth)
        .expect("Can't read line");

    let nth: u128 = nth
        .trim()
        .parse()
        .expect("invalid input");

    println!("The {} Fibonacci number is {:?}", nth, get_fibonacci_number(nth));
}

fn get_fibonacci_number(nth: u128) -> u128 {
    if nth <= 1 {
        return nth
    }

    get_fibonacci_number(nth - 1) + get_fibonacci_number(nth - 2)
}
