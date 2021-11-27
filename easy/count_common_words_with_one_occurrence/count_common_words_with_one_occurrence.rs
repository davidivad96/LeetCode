use std::collections::HashMap;

fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    let mut words1_occurrences: HashMap<&String, i32> = HashMap::new();
    let mut words2_occurrences: HashMap<&String, i32> = HashMap::new();
    for word in words1.iter() {
        words1_occurrences.insert(word, (words1_occurrences.get(word).unwrap_or(&0)) + 1);
    }
    for word in words2.iter() {
        words2_occurrences.insert(word, (words2_occurrences.get(word).unwrap_or(&0)) + 1);
    }
    let mut count: i32 = 0;
    for (word, occurrences) in words1_occurrences {
        if occurrences == 1 && words2_occurrences.get(word).unwrap_or(&0) == &1 {
            count += 1
        }
    }
    count
}

fn main() {
    let words1: Vec<String> = vec![
        "leetcode".to_string(),
        "is".to_string(),
        "amazing".to_string(),
        "as".to_string(),
        "is".to_string(),
    ];
    let words2: Vec<String> = vec![
        "amazing".to_string(),
        "leetcode".to_string(),
        "is".to_string(),
    ];
    println!("Number of common words: {:?}", count_words(words1, words2));
}
