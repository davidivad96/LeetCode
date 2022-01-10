fn add_binary(a: String, b: String) -> String {
    let a_decimal = i128::from_str_radix(&a, 2).unwrap();
    let b_decimal = i128::from_str_radix(&b, 2).unwrap();
    format!("{:b}", a_decimal + b_decimal)
}

fn main() {
    let a: String = "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string();
    let b: String = "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string();
    println!("Sum of binary strings: {}", add_binary(a, b));
}
