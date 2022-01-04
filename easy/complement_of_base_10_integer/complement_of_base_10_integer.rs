use std::env;

fn bitwise_complement(n: i32) -> i32 {
    let binary: String = format!("{:b}", n);
    let bitwise: String = binary
        .chars()
        .map(|x| match x {
            '0' => '1',
            '1' => '0',
            _ => x,
        })
        .collect();
    i32::from_str_radix(&bitwise, 2).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: i32 = args[1].clone().parse::<i32>().unwrap();
    let complement: i32 = bitwise_complement(n);
    println!("Bitwise complement: {}", complement);
}
