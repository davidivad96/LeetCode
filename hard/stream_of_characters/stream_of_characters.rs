struct StreamChecker {
    words: Vec<String>,
    stream: String,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        Self {
            words,
            stream: "".to_string(),
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter);
        for word in self.words.iter() {
            if self.stream.ends_with(word) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let words: Vec<String> = vec![
        "ab".to_string(),
        "ba".to_string(),
        "aaab".to_string(),
        "abab".to_string(),
        "baa".to_string(),
    ];
    let mut stream_checker = StreamChecker::new(words);
    println!("Query with 'a' is: {}", stream_checker.query('a'));
    println!("Query with 'a' is: {}", stream_checker.query('a'));
    println!("Query with 'a' is: {}", stream_checker.query('a'));
    println!("Query with 'a' is: {}", stream_checker.query('a'));
    println!("Query with 'a' is: {}", stream_checker.query('a'));
    println!("Query with 'b' is: {}", stream_checker.query('b'));
    println!("Query with 'a' is: {}", stream_checker.query('a'));
    println!("Query with 'b' is: {}", stream_checker.query('b'));
    println!("Query with 'a' is: {}", stream_checker.query('a'));
    println!("Query with 'b' is: {}", stream_checker.query('b'));
    println!("Query with 'b' is: {}", stream_checker.query('b'));
    println!("Query with 'b' is: {}", stream_checker.query('b'));
    println!("Query with 'a' is: {}", stream_checker.query('a'));
}
