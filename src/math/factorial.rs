use std::io;

pub fn launch() {
    println!("FACTORIAL!");
    println!("--------------");
    println!("Input number to return its factorial.");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error reading number");

    let trimmed = input.trim();

    match trimmed.parse::<i32>() {
        Ok(number) => println!("Factorial is: {}", _factorial(&number)),
        Err(..) => {
            println!("Please enter a number");
            launch();
        }
    }
}
fn _factorial(n: &i32) -> i32 {
    if *n == 1 {
        return *n;
    } else {
        return *n * _factorial(&(n - 1));
    }
}

#[test]
fn it_works_factorial() {
    let number = 5;
    let result = _factorial(&number);
    assert_eq!(result, 120);
}