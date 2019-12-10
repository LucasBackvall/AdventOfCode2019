
pub fn solve_a(parts: Vec<isize>) -> isize {
    let mut sum = 0;
    for part in parts {
        sum = sum + fuel(part);
    }
    
    sum
}

pub fn solve_b(parts: Vec<isize>) -> isize {
    let mut sum = 0;
    for part in parts {
        sum = sum + total_fuel(part);
    }
    
    sum
}


fn fuel(weight: isize) -> isize {
    return weight/3 - 2;
}

fn total_fuel(weight: isize) -> isize {
    let fuel = fuel(weight);
    match fuel > 0 {
        true =>
            fuel + total_fuel(fuel),
        false =>
            0
    }
}

