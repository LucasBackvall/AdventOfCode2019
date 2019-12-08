
pub fn parse_int(inp: String) -> Vec<isize> {
    let mut output = Vec::new();
    let rows = inp.split('\n');
    for row in rows {
        if row == "" {
            continue;
        }
        output.push(
            row.parse::<isize>().unwrap()
        );
    }
    
    output
}

pub fn parse_ic(inp: String) -> Vec<isize> {
    let mut output = Vec::new();
    let intcodes: Vec<&str> = inp.split(',').collect();
    for intcode in intcodes {
        let ic: Vec<&str> = intcode.split('\n').collect();
        if ic[0] == "" {
            continue;
        }
        output.push(
            ic[0].parse::<isize>().unwrap()
        );
    }
    
    output
}
