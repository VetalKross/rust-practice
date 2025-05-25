use std::collections::BTreeSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines
        .next().unwrap().unwrap()
        .trim().parse().unwrap();  

    let mut rects = Vec::new();
    let mut xs = BTreeSet::new();
    let mut ys = BTreeSet::new();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (x1, y1, x2, y2) = (nums[0], nums[1], nums[2], nums[3]);

        rects.push((x1, y1, x2, y2));
        xs.insert(x1);
        xs.insert(x2);
        ys.insert(y1);
        ys.insert(y2);
    }
    let xs: Vec<i32> = xs.into_iter().collect();
    let ys: Vec<i32> = ys.into_iter().collect();

    let mut used = vec![vec![false; ys.len() - 1]; xs.len() - 1];
    for &(x1, y1, x2, y2) in &rects {
        let xi1 = xs.binary_search(&x1).unwrap();
        let xi2 = xs.binary_search(&x2).unwrap();
        let yi1 = ys.binary_search(&y1).unwrap();
        let yi2 = ys.binary_search(&y2).unwrap();

        for xi in xi1..xi2 {
            for yi in yi1..yi2 {
                used[xi][yi] = true;
            }
        }
    }

    let mut area: i64 = 0;
    for xi in 0..xs.len() - 1 {
        for yi in 0..ys.len() - 1 {
            if used[xi][yi] {
                let dx = (xs[xi + 1] - xs[xi]) as i64;
                let dy = (ys[yi + 1] - ys[yi]) as i64;
                area += dx * dy;
            }
        }
    }
    println!("{}", area);
}
