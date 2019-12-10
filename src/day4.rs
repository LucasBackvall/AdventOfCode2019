use std::fmt;

pub fn stupid(input: Vec<isize>) -> isize {
    let mut sum = 0;
    for int in input {
        if Password::password(int).valid(true) {
            sum = sum + 1;
        }
    }
    
    println!("END sum: {}", sum);
    sum
}

pub fn solve_a(input: Vec<isize>) -> isize {
    println!("thread start @ {}", input[0]);
    rec(&input[..], false, 0)
}

pub fn solve_b(input: Vec<isize>) -> isize {
    println!("thread start @ {}", input[0]);
    rec(&input[..], true, 0)
}

fn rec(input: &[isize], only_double: bool, sum: isize) -> isize {
    let slice_len = input.len();
    if slice_len == 0 {
        return sum;
    }
    
    let first = input[0];
    let curr = Password::password(first);
    let next = curr.next();
    
    if next - first > slice_len as isize {
        println!("END fist: {}, next: {}, sum: {}", first, next, sum);
        return sum;
    }
    
    if curr.valid(only_double) {
        rec(&input[(next - first) as usize..], only_double, sum + 1)
    } else {
        rec(&input[(next - first) as usize..], only_double, sum)
    }
}

struct Password {
    dgt_0: isize,
    dgt_1: isize,
    dgt_2: isize,
    dgt_3: isize,
    dgt_4: isize,
    dgt_5: isize
}

impl Password {
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
        for i in (0..5).rev() {
            if pw.digit(i + 1) > pw.digit(i) {
                pw.set_digit(
                    i,
                    pw.digit(i + 1)
                );
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
        let mut digit_1 = -1;
        let mut digit_2 = -1;
        let mut double_digit = false;
        
        for i in 0..6 {
            let digit = self.digit(i);
            
            if only_double && digit == digit_1 && digit == digit_2 {
                return false;
            }
            
            if i > 0 {
                if digit > digit_1 { return false }
            }

            if !double_digit {
                if digit == digit_1 { double_digit = true; }
            }
            
            digit_2 = digit_1;
            digit_1 = digit;
        }
        
        
        double_digit
    }
}


impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_int())
    }
}
