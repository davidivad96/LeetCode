use std::cmp;

fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut result: Vec<Vec<i32>> = triangle;
    for i in (0..result.len() - 1).rev() {
        for j in 0..result[i].len() {
            result[i][j] = cmp::min(
                result[i][j] + result[i + 1][j],
                result[i][j] + result[i + 1][j + 1],
            );
        }
    }
    result[0][0]
}

fn main() {
    let triangle: Vec<Vec<i32>> = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    println!("Minimum path sum of triangle: {}", minimum_total(triangle))
}
