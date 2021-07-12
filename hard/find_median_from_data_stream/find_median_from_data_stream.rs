struct MedianFinder {
    nums: Vec<i32>
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            nums: vec![]
        }
    }
    
    fn add_num(&mut self, num: i32) {
        let pos = self.nums.binary_search(&num).unwrap_or_else(|e| e);
        self.nums.insert(pos, num);
        self.nums.sort();
    }
    
    fn find_median(&self) -> f64 {
        if self.nums.len() % 2 == 0 {
            let middle_num_1 = self.nums[self.nums.len() / 2] as f64;
            let middle_num_2 = self.nums[self.nums.len() / 2 - 1] as f64;
            return f64::from((middle_num_1 + middle_num_2) / 2.0);
        }
        f64::from(self.nums[self.nums.len() / 2])
    }
}

fn main() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(1);
    median_finder.add_num(2);
    println!("Median is: {}", median_finder.find_median());
    median_finder.add_num(3);
    println!("Median is: {}", median_finder.find_median());
}
