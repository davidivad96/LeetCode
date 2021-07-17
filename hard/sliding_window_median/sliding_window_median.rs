fn calculate_median(mut nums: Vec<i32>) -> f64 {
    nums.sort();
    let len = nums.len();
    let mid = len / 2;
    if len % 2 != 0 {
        return f64::from(nums[mid]);
    }
    (f64::from(nums[mid]) + f64::from(nums[mid - 1])) / 2.0
}

fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let mut result: Vec<f64> = vec![];
    for i in 0..(nums.len() - k as usize + 1) {
        result.push(calculate_median((&nums[i..i + k as usize]).to_vec()));
    }
    result
}

fn main() {
    let nums: Vec<i32> = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k: i32 = 3;
    println!(
        "Median array for each window: {:?}",
        median_sliding_window(nums, k)
    );
}
