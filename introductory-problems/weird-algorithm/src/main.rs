use std::io::stdin;

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn main() {
    let mut t = take_int();
    print!("{} ", t);
    while t > 1 {
        match t % 2 {
            1 => t = 3 * t + 1,
            _ => t /= 2,
        }
        print!("{} ", t);
    }
}
