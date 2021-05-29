fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums_sorted = nums.to_vec();
    nums_sorted.sort();
    let n = nums_sorted.len();
    let mut result = nums_sorted[0] + nums_sorted[n - 1];
    let mut i = 1;
    let mut j = n - 2;
    while i <= j {
        let sum = nums_sorted[i] + nums_sorted[j];
        if sum > result {
            result = sum;
        }
        i += 1;
        j -= 1;
    }
    result
}

fn main() {
    let nums: Vec<i32> = vec![3, 5, 4, 2, 4, 6];
    println!("Minimum of the maximum pairs: {}", min_pair_sum(nums));
}
