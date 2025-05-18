const WIDTH: usize = 7;
const HEIGHT: usize = 7;

fn main() {
    let mut output = String::new();

    for i in 0..HEIGHT {
        let stars = if i <= HEIGHT / 2 {
            1 + i * 2
        } else {
            1 + (HEIGHT - 1 - i) * 2
        };
        let spaces = (WIDTH * 2 - 1 - stars) / 2;

        for _ in 0..spaces {
            output.push(' ');
        }
        for _ in 0..stars {
            output.push('★');
        }
        output.push('\n');
    }

    print!("{}", output);
}