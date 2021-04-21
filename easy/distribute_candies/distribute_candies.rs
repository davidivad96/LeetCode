use std::collections::HashSet;

fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let n: usize = candy_type.len() / 2;
    let mut types_seen: HashSet<i32> = HashSet::new();
    let mut different_candies: usize = 0;
    for candy in candy_type {
        if !types_seen.contains(&candy) {
            if different_candies == n - 1 {
                return n as i32;
            }
            different_candies += 1;
            types_seen.insert(candy);
        }
    }
    different_candies as i32
}

fn main() {
    let candy_type: Vec<i32> = vec![1, 1, 2, 2, 3, 3];
    println!(
        "Maximum number of different types of candies: {}",
        distribute_candies(candy_type)
    );
}
