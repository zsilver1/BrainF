use std::collections::HashMap;

pub struct Tape {
    tape: [u8; 10000],
    pointer: usize,
    cur_op: usize,
    ops: Vec<Op>,
    map: HashMap<usize, usize>
}

impl Tape {
    fn new(new_ops: Vec<Op>, new_map: HashMap<usize, usize>) -> Tape {
        Tape {
            tape: [0; 10000],
            pointer: 0,
            cur_op: 0,
            ops: new_ops,
            map: new_map
        }
    }

    fn inc(&mut self) {
        self.pointer += 1;
    }

    fn dec(&mut self) {
        self.pointer -= 1;
    }

    fn inc_byte(&mut self) {
        self.tape[self.pointer] += 1;
    }

    fn dec_byte(&mut self) {
        self.tape[self.pointer] -= 1;
    }

    fn output(&self) {
        println!("{}", self.tape[self.pointer] as char);
    }

    fn input(&mut self) {
        
    }

    fn jump_f(&mut self) {
        
    }

    fn jump_b(&mut self) {
        
    }

    fn finished(&self) -> bool {
        self.cur_op >= ops.len()
    }

    fn execute_instruction(&self) {
        
    }
}

#[derive(Debug)]
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


pub fn run(code: &str) -> Result<(), String> {
    let mut jump_map = HashMap::new();
    let ops: Vec<Op> = parse(code, &mut jump_map)?;
    println!("{:?}", ops);
    let mut tape = Tape::new(ops, jump_map);
    while (!tape.finished()) {
        tape.execute_instruction();
    }
    Ok(())
}

pub fn parse(code: &str,
             map: &mut HashMap<usize, usize>) -> Result<Vec<Op>, String> {
    let mut ops = Vec::new();
    let mut jumps: Vec<usize> = Vec::new();

    let mut counter: usize = 0;
    for c in code.chars() {
        let o: Op = match c {
            '>' => Op::Inc,
            '<' => Op::Dec,
            '+' => Op::IncByte,
            '-' => Op::DecByte,
            '.' => Op::Output,
            ',' => Op::Input,

            '[' => parse_jump_f(counter, &mut jumps),

            ']' => parse_jump_b(counter, &mut jumps, map),

            ' ' | '\n' => continue,
            _ => return Err(format!("Invalid character {} at {}", c, counter))
        };
        ops.push(o);
        counter += 1;
    }
    println!("Jump Map: {:?}\n", map);
    Ok(ops)
}


fn parse_jump_f(counter: usize,
                v: &mut Vec<usize>) -> Op {
    v.push(counter);
    Op::JumpF
}

fn parse_jump_b(counter: usize,
                v: &mut Vec<usize>,
                map: &mut HashMap<usize, usize>) -> Op {
    match v.pop() {
        Some(c) => {
            map.insert(counter, c + 1);
            map.insert(c, counter + 1);
        }
        None => panic!("unbalanced parens at {}", counter)
    }
    Op::JumpB
}
