use std::io;

pub fn launch() {
    println!("PRIME NUMBER!");
    println!("--------------");
    println!(
        "Input two number separeted by ; to return how many numbers is prime in this interval."
    );

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error reading two number");

    let trimmed = input.trim();
    let interval: Vec<&str> = trimmed.split(';').collect();
    let interval_number: Vec<i32> = interval
        .iter()
        .map(|v| v.trim().parse::<i32>().unwrap())
        .collect();
    let interval_number_array: [i32; 2] = [interval_number[0], interval_number[1]];

    println!("Primes numbers in interval are: {:?}", _prime(&interval_number_array));
}

fn _prime(interval: &[i32; 2]) -> Vec<i32> {
    let [initial_interval, final_interval] = interval;
    let initial: i32 = if *initial_interval < 2 { 2 } else { *initial_interval };

    let mut vec_prime: Vec<i32> = (initial..=*final_interval).collect();
    for number in initial..=*final_interval {
        vec_prime.retain(|&x| (x <= number || x % number != 0));
    }
    return vec_prime;
}

#[test]
fn it_works_prime() {
    let interval = [2, 10];
    let result = _prime(&interval);
    assert_eq!(result, vec![2, 3, 5, 7]);
}