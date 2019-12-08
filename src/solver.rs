use std::thread;

pub fn given_ic<F>(solution: F, data: Vec<isize>) -> isize
where F: FnOnce(Vec<isize>) -> isize
{
    solution(data)
}

pub fn sum<F>(
    num_threads: usize, 
    input: Vec<isize>, 
    solution: F
) -> isize
where F: FnOnce(isize) -> isize + Send + Copy + 'static
{
    if num_threads < 2 {
        sum_thread(input, solution)
        
    } else {
        let mut threads = Vec::new();
        
        let size = input.len()/num_threads;
        for n in 0..num_threads {
            let pos = n*size;
            let mut slice: Vec<isize>;
            
            if n+1 == num_threads {
                let size = input.len() - pos;
                slice = vec![0; size];
                slice.copy_from_slice(&input[pos..input.len()]);
            } else {
                slice = vec![0; size];
                slice.copy_from_slice(&input[pos..pos+size]);
            }
            
            threads.push(
                thread::spawn(move || -> isize {
                    sum_thread(slice, solution)
                })
            );
        }
        
        let mut sum = 0;
        for thread in threads {
            sum = sum + thread.join().unwrap();
        }
        
        sum
    }
}

fn sum_thread<F>(inp: Vec<isize>, solution: F) -> isize
    where F: FnOnce(isize) -> isize + Send + Copy + 'static
{
    let mut sum = 0;
    for row in inp {
        sum = sum + solution(row);
    }
    
    sum
}
