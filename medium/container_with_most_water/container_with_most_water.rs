use std::{cmp};

fn calculate_area(arr: &Vec<i32>, i: usize, j: usize) -> i32 {
    ((j - i) as i32) * cmp::min(arr[i], arr[j])
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut left_index: usize = 0;
    let mut right_index: usize = height.len() - 1;
    let mut max: i32 = -1;
    while left_index != right_index {
        let area: i32 = calculate_area(&height, left_index, right_index);
        if area > max { max = area; }
        if height[left_index] < height[right_index] { left_index += 1; }
        else { right_index -= 1; }
    }
    max
}

fn main() {
    let height: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    println!("Max area of water: {}", max_area(height));
}
