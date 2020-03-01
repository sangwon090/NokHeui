use std::process;
use std::env;
use std::fs;

use nokheui::Nokheui;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: ./nokheui [options] filename");
        process::exit(-1);
    }

    let data: String = fs::read_to_string(&args[1]).unwrap();
    let data: Vec<Vec<char>> = data.split('\n').map(|line| line.chars().collect()).collect();

    let mut interpreter: Nokheui = Nokheui::new(data);
    let exit_code: i32 = interpreter.run();

    process::exit(exit_code);
}