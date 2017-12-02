use std::env;
use std::fs::File;
use std::io::prelude::*;

use lib::*;
mod lib;



fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("wrong number of arguments");
    let mut f = File::open(filename).expect("file not found");

    let mut code = String::new();
    f.read_to_string(&mut code).expect("error reading file");

    match run(&code) {
        Err(e) => println!("{}\n", e),
        Ok(()) => ()
    }
}
