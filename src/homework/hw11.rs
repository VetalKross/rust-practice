use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32) {
    let mut min_index = 0;
    let mut min_sum = data[0] + data[1];
    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }
    (min_index, min_sum)
}

fn print_vector_with_min_sum(data: &[i32]) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    print!("data:    [");
    for (i, val) in data.iter().enumerate() {
        if i < data.len() - 1 {
            print!("{:>2}, ", val);
        } else {
            print!("{:>2}", val);
        }
    }
    println!("]");

    let (min_index, min_sum) = min_adjacent_sum(data);

    print!("indexes: ");
    for i in 0..(min_index * 4 + 1) {
        print!(" ");
    }
    println!("\\__ __/");

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_index],
        data[min_index + 1],
        min_sum,
        min_index,
        min_index + 1
    );
}

fn main() {
    let vec = gen_random_vector(20);
    print_vector_with_min_sum(&vec);
}
