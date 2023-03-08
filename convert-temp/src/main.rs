use std::io;

fn main() {
    let option = get_option();

    let temp = get_temp(option);

    if option == 'F' {
        println!("Converting {} degrees from Farhenheit to Celsius...", temp);
        let result = farhenheit_to_celsius(temp);
        println!("{} degrees Farhenheit in Celsius is {}", temp, result);
    } else {
        println!("Converting {} degrees from Celsius to Farhenheit...", temp);
        let result = celcius_to_farhenheit(temp);
        println!("{} degrees Celsius in Farhenheit is {}", temp, result);
    }
}

fn farhenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0 / 9.0)
}

fn celcius_to_farhenheit(temp: f64) -> f64 {
    (temp * (9.0 / 5.0)) + 32.0
}

fn get_option() -> char {
    let mut option = String::new();

    loop {
        option.clear();

        println!("Please enter F or C:");

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option = option.trim().to_uppercase();

        if option == "F" || option == "C" {
            return option.chars().next().unwrap();
        }
    }
}

fn get_temp(option: char) -> f64 {
    let mut temp = String::new();

    loop {
        temp.clear();

        println!("Enter the temperature in {}:", {
            if option == 'F' {
                "Farhenheit"
            } else {
                "Celcius"
            }
        });

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temperature");

        let temp: f64 = match temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("Temperature should be a floating point number!");
                continue;
            }
        };

        return temp;
    }
}
