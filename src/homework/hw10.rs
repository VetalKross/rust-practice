fn is_palindrome(x: u32) -> bool {
    // Переводим число в строку
    let s = x.to_string();
    s.chars().rev().collect::<String>() == s
}

fn main() {
    let nums = [123, 121, 1221, 12321, 77, 5, 10];
    for &n in &nums {
        println!("{}: {}", n, is_palindrome(n));
    }
}

#[test]
fn test() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];

    data.iter().for_each(|(n, exp)| {
        assert_eq!(is_palindrome(*n), *exp);
    });
}
