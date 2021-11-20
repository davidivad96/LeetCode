fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut index: usize = 0;
    while index < nums.len() - 1 {
        if nums[index] == nums[index + 1] {
            index += 2;
        } else {
            return nums[index];
        }
    }
    return nums[index];
}

fn main() {
    let nums: Vec<i32> = vec![1, 1, 2, 2, 3, 3, 4, 4, 8];
    println!("Single element: {}", single_non_duplicate(nums));
}
