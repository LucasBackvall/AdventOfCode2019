pub fn solve_a(weight: isize) -> isize {
    return weight/3 - 2;
}

pub fn solve_b(weight: isize) -> isize {
    let fuel = solve_a(weight);
    match fuel > 0 {
        true =>
            fuel + solve_b(fuel),
        false =>
            0
    }
}

