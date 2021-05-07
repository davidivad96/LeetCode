fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    let n: usize = nums.len();
    let mut nums_copy: Vec<i32> = nums.to_vec();
    nums_copy.sort();
    let mut i: usize = 0;
    let mut j: usize = n - 1;
    while nums[i] == nums_copy[i] && i < n - 1 {
        i += 1;
    }
    while nums[j] == nums_copy[j] && j > 1 {
        j -= 1;
    }
    if i == n - 1 || j == 0 {
        return 0;
    }
    (j - i) as i32 + 1
}

fn main() {
    let nums: Vec<i32> = vec![2, 6, 4, 8, 10, 9, 15];
    println!("Shortest subarray length: {}", find_unsorted_subarray(nums));
}
