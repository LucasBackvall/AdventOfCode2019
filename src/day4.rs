use std::fmt;
use std::ops::Range;

pub fn solve_a(input: Range<isize>) -> isize
{
    println!("Thread start @ {}..{}", input.start, input.end);
    run(input, false)
}

pub fn solve_b(input: Range<isize>) -> isize
{
    println!("Thread start @ {}..{}", input.start, input.end);
    run(input, true)
}

fn run(input: Range<isize>, only_double: bool) -> isize
{
    let mut sum = 0;
    let mut lower = input.start;

    while lower < input.end {
        let pw = Password::password(lower);
        if pw.valid(only_double) {
            sum = sum + 1;
        }
        
        lower = pw.next();
    }
    
    sum
}

struct Password
{
    dgt_0: isize,
    dgt_1: isize,
    dgt_2: isize,
    dgt_3: isize,
    dgt_4: isize,
    dgt_5: isize
}

impl Password
{
    pub fn password(inp: isize) -> Password {
        Password {
            dgt_0: inp % 10,
            dgt_1: (inp / 10) % 10,
            dgt_2: (inp / 100) % 10,
            dgt_3: (inp / 1000) % 10,
            dgt_4: (inp / 10000) % 10,
            dgt_5: (inp / 100000) % 10
        }
    }
    
    pub fn next(&self) -> isize {
        let mut pw = Password::password(self.as_int());
        let mut carries = false;
        for i in (0..5).rev() {
            if pw.digit(i + 1) > pw.digit(i) || carries {
                pw.set_digit(
                    i,
                    pw.digit(i + 1)
                );
                carries = true;
            }
        }
        
        if pw.as_int() == self.as_int() {
            return self.as_int() + 1;
        }
        
        pw.as_int()
    }

    pub fn as_int(&self) -> isize {
        self.dgt_0 +
        self.dgt_1 * 10 +
        self.dgt_2 * 100 +
        self.dgt_3 * 1000 +
        self.dgt_4 * 10000 +
        self.dgt_5 * 100000
    }

    pub fn set_digit(&mut self, digit: isize, val: isize) {
        match digit {
            0 => self.dgt_0 = val,
            1 => self.dgt_1 = val,
            2 => self.dgt_2 = val,
            3 => self.dgt_3 = val,
            4 => self.dgt_4 = val,
            5 => self.dgt_5 = val,
            _ => panic!("Not a valid Password digit.")
        }
    }

    pub fn digit(&self, digit: isize) -> isize {
        match digit {
            0 => self.dgt_0,
            1 => self.dgt_1,
            2 => self.dgt_2,
            3 => self.dgt_3,
            4 => self.dgt_4,
            5 => self.dgt_5,
            _ => panic!("Not a valid Password digit.")
        }
    }
    
    pub fn valid(&self, only_double: bool) -> bool {
        let mut same: Vec<isize> = Vec::new();
        let mut double_digit = false;
        let mut last_digit = 10;
        
        for i in 0..6
        {
            let digit = self.digit(i);
            if digit > last_digit { return false; }
            
            if only_double {
                if digit != last_digit
                {
                    if same.len() == 2 {
                        double_digit = true;
                    }
                    same = Vec::new();
                    same.push(digit);
                }
                else
                {
                    same.push(digit);
                }
            }
            else if !double_digit && digit == last_digit {
                double_digit = true;
            }
            
            last_digit = digit;
        }
        if only_double && same.len() == 2 {
            return true;
        }
        
        double_digit
    }
}


impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_int())
    }
}
