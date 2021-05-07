use std::collections::HashSet;
use std::env;

fn sum_square_digits(n: i32) -> u32 {
    let sum: u32 = n
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap().pow(2))
        .fold(0, |a, x| a + x);
    return sum;
}

fn is_happy(n: i32) -> bool {
    if n == 1 {
        return true;
    }
    let mut memory: HashSet<u32> = HashSet::new();
    let mut sum: u32 = sum_square_digits(n);
    while sum != 1 {
        if memory.contains(&sum) {
            return false;
        } else {
            memory.insert(sum);
            sum = sum_square_digits(sum as i32);
        }
    }
    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: i32 = args[1].clone().parse::<i32>().unwrap();
    let is_happy_number: bool = is_happy(n);

    if is_happy_number {
        println!("{} IS a happy number", n);
    } else {
        println!("{} IS NOT a happy number", n);
    }
}
