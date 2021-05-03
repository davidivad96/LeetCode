fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut sum: i32 = 0;
    for num in nums {
        sum += num;
        result.push(sum);
    }
    result
}

fn main() {
    let nums: Vec<i32> = vec![3, 1, 2, 10, 1];
    println!("Running sum: {:?}", running_sum(nums));
}
