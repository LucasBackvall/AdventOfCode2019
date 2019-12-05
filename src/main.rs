use std::fs;

fn main() {
    let inp = get_input(1);
    println!("{}", inp);
}

fn get_input(n: u8) -> String {
    let addr = format!("input/day{}.txt", n.to_string());
    return fs::read_to_string(addr).expect("failed to read file");
}
