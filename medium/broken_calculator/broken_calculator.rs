use std::env;

fn broken_calc(x: i32, y: i32) -> i32 {
    let mut sum: i32 = 0;
    let x_copy: i32 = x;
    let mut y_copy: i32 = y;
    while x_copy < y_copy {
        if y_copy % 2 == 0 {
            y_copy /= 2;
        } else {
            y_copy += 1;
        }
        sum += 1;
    }
    sum + x_copy - y_copy
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let x: i32 = args[1].clone().parse::<i32>().unwrap();
    let y: i32 = args[2].clone().parse::<i32>().unwrap();
    let number_of_operations: i32 = broken_calc(x, y);

    println!("Number of operations needed: {}", number_of_operations);
}
