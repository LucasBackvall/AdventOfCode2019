pub struct Computer {
    memory: Vec<isize>,
    program_counter: usize,
    running: bool
}

pub trait ExecOpcode {
    fn exec_opcode(&mut self);
}

pub trait RunProgram: ExecOpcode {
    fn run_program(&mut self);
}

impl Computer {
    pub fn computer(data: Vec<isize>) -> Computer {
        Computer {
            memory: data,
            program_counter: 0,
            running: false
        }
    }

    pub fn get_data(&self, address: usize) -> isize {
        self.memory[address]
    }

    pub fn set_data(&mut self, address: usize, inp: isize) {
        self.memory[address] = inp;
    }
}

impl ExecOpcode for Computer {
    fn exec_opcode(&mut self) {
        let pc = self.program_counter;
        
        let instruction = self.memory[pc];
        let a = self.memory[pc + 1] as usize;
        let b = self.memory[pc + 2] as usize;
        let target = self.memory[pc + 3] as usize;
        
        match instruction {
            1 =>
                self.memory[target] = self.memory[a] + self.memory[b],
            2 =>
                self.memory[target] = self.memory[a] * self.memory[b],
            99 =>
                self.running = false,
            _ =>
                panic!("Invalid OP-code")
        }
        
        self.program_counter = pc + 4;
    }
}

impl RunProgram for Computer {
    fn run_program(&mut self) {
        self.running = true;
        while self.running {
            self.exec_opcode()
        }
    }
}
