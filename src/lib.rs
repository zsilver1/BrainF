use std::collections::HashMap;
use std::io::{self, Read};

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
        print!("{}", self.tape[self.pointer] as char);
    }

    fn input(&mut self) {
        let input = io::stdin()
            .bytes() 
            .next()
            .and_then(|result| result.ok());
        match input {
            None => panic!("input error"),
            Some(c) => self.tape[self.pointer] = c
        }
    }

    fn jump_f(&mut self) {
        if self.tape[self.pointer] == 0 {
            self.cur_op = match self.map.get(&self.cur_op) {
                Some(i) => *i,
                None => panic!("forward jump not found")
            };
        }
    }

    fn jump_b(&mut self) {
        if self.tape[self.pointer] != 0 {
            self.cur_op = match self.map.get(&self.cur_op) {
                Some(i) => *i,
                None => panic!("backward jump not found")
            };
        }
    }

    fn finished(&self) -> bool {
        self.cur_op >= self.ops.len()
    }

    fn execute_instruction(&mut self) {
        let op = *self.ops.get(self.cur_op).expect("instruction error");
        self.perform_op(op);
        self.cur_op += 1;
    }

    fn perform_op(&mut self, op: Op) {
        match op {
            Op::Inc => self.inc(),
            Op::Dec => self.dec(),
            Op::IncByte => self.inc_byte(),
            Op::DecByte => self.dec_byte(),
            Op::Output => self.output(),
            Op::Input => self.input(),
            Op::JumpF => self.jump_f(),
            Op::JumpB => self.jump_b(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

pub fn run(code: &str) -> Result<(), String> {
    let mut jump_map = HashMap::new();
    let ops: Vec<Op> = parse(code, &mut jump_map)?;
    let mut tape = Tape::new(ops, jump_map);
    while !tape.finished() {
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
            map.insert(counter, c);
            map.insert(c, counter);
        }
        None => panic!("unbalanced parens at {}", counter)
    }
    Op::JumpB
}
