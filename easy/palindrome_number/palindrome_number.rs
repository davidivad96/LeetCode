use std::env;

fn is_palindrome(x: i32) -> bool {
    let string: String = x.to_string();
    let reversed_string: String = string.chars().rev().collect();
    string == reversed_string
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let x: i32 = args[1].clone().parse::<i32>().unwrap();
    let is_palindrome_number: bool = is_palindrome(x);

    if is_palindrome_number {
        println!("{} IS a palindrome number", x);
    } else {
        println!("{} IS NOT a palindrome number", x);
    }
}
