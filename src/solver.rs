use std::thread;

pub fn int_range<F>(
    num_threads: usize, 
    input: Vec<isize>, 
    solution: F
) -> isize
where F: FnOnce(Vec<isize>) -> isize + Send + Copy + 'static
{
    if num_threads < 2 {
        solution(input)
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
                    solution(slice)
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
