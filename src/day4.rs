use std::ops::Range;
use std::convert::TryInto;

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
    let mut setup = true;
    
    let start_1: u8 = (input.start % 10).try_into().unwrap();
    let start_2: u8 = ((input.start / 10) % 10).try_into().unwrap();
    let start_3: u8 = ((input.start / 100) % 10).try_into().unwrap();
    let start_4: u8 = ((input.start / 1000) % 10).try_into().unwrap();
    let start_5: u8 = ((input.start / 10000) % 10).try_into().unwrap();
    let start_6: u8 = ((input.start / 100000) % 10).try_into().unwrap();
    
    let end_1: u8 = (input.end % 10).try_into().unwrap();
    let end_2: u8 = ((input.end / 10) % 10).try_into().unwrap();
    let end_3: u8 = ((input.end / 100) % 10).try_into().unwrap();
    let end_4: u8 = ((input.end / 1000) % 10).try_into().unwrap();
    let end_5: u8 = ((input.end / 10000) % 10).try_into().unwrap();
    let end_6: u8 = ((input.end / 100000) % 10).try_into().unwrap();
    
    let mut last_6 = false;
    let mut last_5 = false;
    let mut last_4 = false;
    let mut last_3 = false;
    let mut last_2 = false;
    
    
    for d6 in 0..10 as u8 {
        if setup && d6 < start_6 { continue; }
        if d6 == end_6 { last_6 = true; }
        if d6 > end_6 {
            println!("Break at {}{}{}{}{}{}",
                d6, d6, d6, d6, d6, d6);
            break;
        }
        
        for d5 in d6..10 as u8 {
            if setup && d5 < start_5 { continue; }
            if last_6 && d5 == end_5 { last_5 = true; }
            if last_6 && d5 > end_5 {
                println!("Break at {}{}{}{}{}{}",
                    d6, d5, d5, d5, d5, d5);
                break;
            }
            
            for d4 in d5..10 as u8 {
                if setup && d4 < start_4 { continue; }
                if last_5 && d4 == end_4 { last_4 = true; }
                if last_5 && d4 > end_4 {
                    println!("Break at {}{}{}{}{}{}",
                        d6, d5, d4, d4, d4, d4);
                    break;
                }
                
                for d3 in d4..10 as u8 {
                    if setup && d3 < start_3 { continue }
                    if last_4 && d3 == end_3 { last_3 = true; }
                    if last_4 && d3 > end_3 {
                        println!("Break at {}{}{}{}{}{}",
                            d6, d5, d4, d3, d3, d3);
                        break;
                    }
                    
                    for d2 in d3..10 as u8 {
                        if setup && d2 < start_2 { continue; }
                        if last_3 && d2 == end_2 { last_2 = true;}
                        if last_3 && d2 > end_2 {
                            println!("Break at {}{}{}{}{}{}",
                                d6, d5, d4, d3, d2, d2);
                            break;
                        }
                        
                        for d1 in d2..10 as u8 {
                            if setup && d1 < start_1 { continue; }
                            if setup {
                                println!("Started at {}{}{}{}{}{}",
                                    d6, d5, d4, d3, d2, d1);
                                setup = false;
                            }
                            if last_2 && d1 == end_1 {
                                println!("Break at {}{}{}{}{}{}",
                                    d6, d5, d4, d3, d2, d1);
                                break;
                            }
                            
                            if has_double(d6, d5, d4, d3, d2, d1, only_double) {
                                sum = sum + 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    sum
}

fn has_double(d6: u8, d5: u8, d4: u8, d3: u8, d2: u8, d1: u8, only_double: bool) -> bool {
    only_double && (
        d6 == d5 && d5 != d4 // XXxxxx
        || d6 != d5 && d5 == d4 && d4 != d3 // xXXxxx
        || d5 != d4 && d4 == d3 && d3 != d2 // xxXXxx
        || d4 != d3 && d3 == d2 && d2 != d1 // xxxXXx
        || d3 != d2 && d2 == d1 // xxxxXX
    )
    || !only_double && (
        d6 == d5
        || d5 == d4
        || d4 == d3
        || d3 == d2
        || d2 == d1
    )
}

