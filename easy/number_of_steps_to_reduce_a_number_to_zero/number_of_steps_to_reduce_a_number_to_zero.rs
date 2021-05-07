use std::env;

fn number_of_steps(num: i32) -> i32 {
    let mut num_copy: i32 = num;
    let mut cont: i32 = 0;
    while num_copy != 0 {
        if num_copy % 2 == 0 {
            num_copy /= 2
        } else {
            num_copy -= 1
        }
        cont += 1;
    }
    cont
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let num: i32 = args[1].clone().parse::<i32>().unwrap();
    println!("Number of steps: {}", number_of_steps(num));
}
