const W: u32 = 25;
const H: u32 = 10;

fn main() {
    let mut result = String::new();

    for y in 0..H {
        for x in 0..W {
            if y == 0 || y == H - 1 || x == 0 || x == W - 1 || x == y * (W - 1) / (H - 1) || x == (H - 1 - y) * (W - 1) / (H - 1) {
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    print!("{}", result);
}