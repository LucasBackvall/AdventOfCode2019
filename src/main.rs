use std::fs;
use std::env;

mod solver;
mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem: usize = args[1][..].parse::<usize>().unwrap();
    let subproblem: usize = args[2][..].parse::<usize>().unwrap();
    let num_threads: usize = args[3][..].parse::<usize>().unwrap();
    
    let input = get_input(problem);
    let data = parse(input);

    
    let solution;
    match (problem, subproblem) {
        (1, 1) =>
            solution = solver::sum(
                num_threads,
                data,
                &day1::solve_a
            ),
        (1, 2) =>
            solution = solver::sum(
                num_threads,
                data,
                &day1::solve_b
            ),
        _ => return
    }
    
    println!("{}", solution);
}

fn get_input(n: usize) -> String {
    let addr = format!("input/day{}.txt", n.to_string());
    return fs::read_to_string(addr).expect("failed to read file");
}

fn parse(inp: String) -> Vec<isize> {
    let mut output = Vec::new();
    let split = inp.split('\n');
    for row in split {
        if row == "" {
            continue;
        }
        output.push(
            row.parse::<isize>().unwrap()
        );
    }
    
    output
}
