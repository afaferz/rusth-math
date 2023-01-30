use std::io;
use math_rs::math::factorial;
use math_rs::math::prime;

fn main() {
    _menu();
}

fn _menu() {
    println!("Select an operation");
    println!("========================");
    println!("\n 1 - FACTORIAL \n 2 - PRIME NUMBERS");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error reading number");

    let trimmed = input.trim();

    match trimmed.parse::<i32>() {
        Ok(n) =>
            match n {
                1 => factorial::launch(),
                2 => prime::launch(),
                _ => _menu(),
            }
        Err(..) => {
            println!("Please enter a valid operation");
            _menu();
        }
    }
}