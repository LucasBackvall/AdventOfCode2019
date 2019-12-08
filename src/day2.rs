use crate::ic_computer;
use crate::ic_computer::RunProgram;

pub fn solve_a(data: Vec<isize>) -> isize {
    let mut comp = ic_computer::Computer::computer(data);
    comp.set_data(1, 12);
    comp.set_data(2, 2);
    comp.run_program();
    comp.get_data(0)
}
