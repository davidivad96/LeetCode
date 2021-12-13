fn max_power(s: String) -> i32 {
    let mut max: i32 = 1;
    let mut sum: i32 = 1;
    let mut i: usize = 0;
    while i < s.len() - 1 {
        let current: char = s.chars().nth(i).unwrap();
        let next: char = s.chars().nth(i + 1).unwrap();
        if current == next {
            sum += 1;
            if sum > max {
                max = sum
            }
        } else {
            sum = 1
        }
        i += 1;
    }
    max
}

fn main() {
    let s: String = "hooraaaaaaaaaaay".to_string();
    println!("Power of s: {}", max_power(s));
}
