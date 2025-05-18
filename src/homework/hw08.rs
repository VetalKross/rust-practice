fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }

    if *n == 2 {
        return true;
    }
    if *n == 3 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;

    let mut is_prime = true;

    while i * i <= *n {
        if *n % i == 0 {
            is_prime = false;
            break;
        }
        i += 1;
    }

    if is_prime == true {
        return true;
    } else {
        return false;
    }
}

// Тест
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_prime() {
        let test_data = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (100, false),
            (10007, true),
        ];

        test_data.iter().for_each(|(n, prime)| {
            assert_eq!(is_prime(n), *prime)
        })
    }
}
