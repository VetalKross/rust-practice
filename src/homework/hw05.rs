fn gcd(a: u32, b: u32) -> u32 {
    let mut m = a;
    let mut n = b;

    while n > 0 {
        println!("gcd step: m = {}, n = {}", m, n);
        let r = m % n;
        m = n;
        n = r;
    }

    m  
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_cases() {
        let data = [
            ((24, 60), 12),
            ((15, 9), 3),
            ((15, 6), 3),
            ((140, 40), 20),
            ((24, 16), 8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37, 11), 1),
            ((120, 90), 30),
        ];

        for ((a, b), exp) in data.iter() {
            assert_eq!(*exp, gcd(*a, *b));
        }
    }
}

fn main() {
    
    println!("GCD of 1071 and 1029 = {}", gcd(1071, 1029));
}
