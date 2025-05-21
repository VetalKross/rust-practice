fn rotate(s: String, n: isize) -> String {
    let len = s.len();

    if len == 0 {
        return s;
    }
    let mut shift = n % len as isize;
    if shift < 0 {
        shift += len as isize;
    }
    let shift = shift as usize;
    let left = &s[0..len - shift];
    let right = &s[len - shift..];
    format!("{}{}", right, left)
}

fn main() {
    let s = "abcdefgh".to_string();
    let n = 2;
    let result = rotate(s, n);
    println!("{}", result);
}

#[test]
fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| 
        assert_eq!(
            rotate(s.clone(), *n), 
            exp.to_string()
        )
    );
     fn main() {
    let s = "abcdefgh".to_string();
    let n = 2;
    let result = rotate(s, n);
    println!("{}", result); 
}
}
