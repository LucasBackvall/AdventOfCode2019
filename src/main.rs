use std::fs;
use std::env;
use std::ops::Range;

mod solver;
mod day1;
mod day4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem: usize = args[1][..].parse::<usize>().unwrap();
    let subproblem: usize = args[2][..].parse::<usize>().unwrap();
    let num_threads: usize = args[3][..].parse::<usize>().unwrap();
    
    let input = get_input(problem);
    
    let solution;
    match (problem, subproblem) {
        (1, 1) =>
            solution = solver::int_vector(
                num_threads,
                parse(input),
                &day1::solve_a
            ),
        (1, 2) =>
            solution = solver::int_vector(
                num_threads,
                parse(input),
                &day1::solve_b
            ),
        (4, 1) =>
            solution = solver::int_range(
                num_threads,
                get_range(input),
                &day4::solve_a
            ),
        (4, 2) =>
            solution = solver::int_range(
                num_threads,
                get_range(input),
                &day4::solve_b
            ),
        _ => return
    }
    
    println!("{}", solution);
}

fn get_input(n: usize) -> String {
    let addr = format!("input/day{}.txt", n.to_string());
    let exception_message = format!("failed to read file \"./input/day{}.txt\"", n.to_string());
    return fs::read_to_string(addr).expect(&exception_message);
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

fn get_range(inp: String) -> Range<isize> {
    let input: Vec<&str> = inp.split('\n').collect();
    let params: Vec<&str> = input[0].split('-').collect();
    
    let a = params[0].parse::<isize>().unwrap();
    let b = params[1].parse::<isize>().unwrap();
    
    a..b
}




