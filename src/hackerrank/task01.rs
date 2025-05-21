use std::io;

fn simple_array_sum(ar: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..ar.len() {
        sum += ar[i];
    }
    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap(); 

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = simple_array_sum(&numbers);
    println!("{}", result);
}
