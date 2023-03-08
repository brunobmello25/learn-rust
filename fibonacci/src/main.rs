use std::io;

fn main() {
    let number = get_number();

    let fib_result = fib(number);

    println!("The fibonacci number of {} is {}", number, fib_result);
}

fn fib(number: u32) -> u32 {
    if number <= 1 {
        return number;
    };

    return fib(number - 1) + fib(number - 2);
}

fn get_number() -> u32 {
    println!("Enter a number for the fibonacci sequence:");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read number from console!");

    loop {
        let number: u32 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("You must enter a positive integer number!");
                continue;
            }
        };

        return number;
    }
}
