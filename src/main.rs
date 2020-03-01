use std::process;
use std::env;
use std::fs;

use nokheui::hangul::disassemble;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: ./nokheui [options] filename");
        process::exit(-1);
    }

    let data: String = fs::read_to_string(&args[1]).unwrap();

    println!("{:?}", data);    
}