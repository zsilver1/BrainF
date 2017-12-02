pub struct Tape {
    tape: [u8; 10000],
    pointer: usize
}

impl Tape {
    pub fn new() -> Tape {
        Tape {
            tape: [0; 10000],
            pointer: 0
        }
    }

    pub fn inc(&mut self) {
        self.pointer += 1;
    }

    pub fn dec(&mut self) {
        self.pointer -= 1;
    }

    pub fn inc_byte(&mut self) {
        self.tape[self.pointer] += 1;
    }

    pub fn dec_byte(&mut self) {
        self.tape[self.pointer] -= 1;
    }

    pub fn output(&self) {
        println!("{}", self.tape[self.pointer] as char);
    }

    pub fn input(&mut self) {
        
    }

    pub fn jump_f(&mut self) {
        
    }

    pub fn jump_b(&mut self) {
        
    }
}


pub enum Op {
    Inc,
    Dec,
    IncByte,
    DecByte,
    Output,
    Input,
    JumpF,
    JumpB
}

impl Op {
    pub fn perform(&self, tape: &mut Tape) {
        match *self {
            Op::Inc => tape.inc(),
            Op::Dec => tape.dec(),
            Op::IncByte => tape.inc_byte(),
            Op::DecByte => tape.dec_byte(),
            Op::Output => tape.output(),
            Op::Input => tape.input(),
            Op::JumpF => tape.jump_f(),
            Op::JumpB => tape.jump_b(),
        }
    }
}


fn run() {
    
}
