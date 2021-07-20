use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Solution {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&self) -> Vec<i32> {
        let mut result: Vec<i32> = self.nums.clone();
        let len: usize = self.nums.len();
        let mut rng = thread_rng();
        for i in 0..len {
            result.swap(i, rng.gen_range(i..len));
        }
        result
    }
}

fn main() {
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let obj: Solution = Solution::new(nums);
    println!("Obj: {:?}", obj);
    let ret_1: Vec<i32> = obj.shuffle();
    println!("Ret 1: {:?}", ret_1);
    let ret_2: Vec<i32> = obj.reset();
    println!("Ret 2: {:?}", ret_2);
    let ret_3: Vec<i32> = obj.shuffle();
    println!("Ret 3: {:?}", ret_3);
}
