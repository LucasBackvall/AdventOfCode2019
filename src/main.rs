use std::fs;
use std::env;

mod parser;
mod solver;
mod day1;
mod day2;
mod ic_computer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem: usize = args[1][..].parse::<usize>().unwrap();
    let subproblem: usize = args[2][..].parse::<usize>().unwrap();
    let num_threads: usize = args[3][..].parse::<usize>().unwrap();
    
    let input = get_input(problem);

    
    let solution;
    match (problem, subproblem) {
        (1, 1) =>
            solution = solver::sum(
                num_threads,
                parser::parse_int(input),
                &day1::solve_a
            ),
        (1, 2) =>
            solution = solver::sum(
                num_threads,
                parser::parse_int(input),
                &day1::solve_b
            ),
        (2, 1) =>
            solution = solver::given_ic(
                &day2::solve_a,
                parser::parse_ic(input)
            ),
        _ => return
    }
    
    println!("{}", solution);
}

fn get_input(n: usize) -> String {
    let addr = format!("input/day{}.txt", n.to_string());
    return fs::read_to_string(addr).expect("failed to read file");
}
