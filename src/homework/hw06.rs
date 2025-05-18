fn main() {
    let triangles = 4; 
    let max_height = triangles + 1;
    let max_width = 1 + (max_height - 1) * 2;


    let mut lines = Vec::new();

    for t in 0..triangles {
        let height = t + 2;
        (0..height).for_each(|i| {
            let stars = 1 + i * 2;
            let spaces = (max_width - stars) / 2;
            let mut row = String::new();
            row.push_str(&" ".repeat(spaces));
            row.push_str(&"*".repeat(stars));
            lines.push(row);
        });
    }


    lines.iter().for_each(|line| println!("{}", line));
}
