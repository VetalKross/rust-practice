pub fn count_permutation(shipments: &Vec<u32>) -> usize {
    let n = shipments.len();
    let mut total: u32 = 0;
    for &w in shipments {
        total += w;
    }

    if total as usize % n != 0 {
        return 0;
    }

    let avg = total / n as u32;
    let mut moves: usize = 0;
    for &w in shipments {
        if w > avg {
            moves += (w - avg) as usize;
        }
    }

    moves
}

pub fn count_permutation_opt(shipments: &Vec<u32>) -> Option<usize> {
    let n = shipments.len();
    let mut total: u32 = 0;
    for &w in shipments {
        total += w;
    }

    if total as usize % n != 0 {
        return None;
    }

    let avg = total / n as u32;
    let mut moves: usize = 0;
    for &w in shipments {
        if w > avg {
            moves += (w - avg) as usize;
        }
    }

    Some(moves)
}

pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    if n == 0 {
        return v;
    }

    for i in 1..n {
        v.push(i as u32);
    }

    let mut sum: u32 = 0;
    for &w in &v {
        sum += w;
    }

    let target_sum = (n as u32) * (n as u32);
    let last = if target_sum > sum {
        target_sum - sum
    } else {
        0
    };

    v.push(last);
    v
}

fn main() {
    let a = vec![8, 2, 2, 4, 4];
    println!(
        "Для {:?} мінімальні ходи = {}",
        a,
        count_permutation(&a)
    );

    let b = vec![9, 3, 7, 2, 9];
    println!(
        "Для {:?} мінімальні ходи = {}",
        b,
        count_permutation(&b)
    );

    let g = gen_shipments(5);
    println!("Згенеровані вантажі (n=5): {:?}", g);
}
