fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }
    let mut sum = 0;
    for i in 0..nums.len() - 2 {
        let diff = nums[i] - nums[i + 1];
        let mut j = 0;
        while i + j + 2 < nums.len() && nums[i + j + 1] - nums[i + j + 2] == diff {
            sum += 1;
            j += 1;
        }
    }
    sum
}

fn main() {
    let nums: Vec<i32> = vec![3, -1, -5, -9];
    println!(
        "Number of arithmetics subarrays: {}",
        number_of_arithmetic_slices(nums)
    );
}
