use std::collections::VecDeque;

use crate::hangul::{JONGSEONG_LIST};

#[derive(Debug)]
pub struct Storage {
    stacks: [Vec<i32>; 26],
    queue: VecDeque<i32>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            stacks: Default::default(),
            queue: Default::default(),
        }
    }

    pub fn push(&mut self, memory_number: usize, value: i32) {        
        if memory_number <= 25 {
            self.stacks[memory_number].push(value);
        } else if memory_number == 26 {
            self.queue.push_back(value);
        } else if memory_number == 27 {
            eprintln!("[*] Pipe is not implemented.")
        } else {
            eprintln!("[*] #{} is invalid storage.", memory_number);
        }
    }

    pub fn pop(&mut self, memory_number: usize) -> i32 {
        if memory_number <= 25 {
            match self.stacks[memory_number].pop() {
                Some(n) => return n,
                None => {
                    eprintln!("[*] Stack {} is empty.", memory_number);
                    return 0;
                }
            }
        } else if memory_number == 26 {
            match self.queue.pop_front() {
                Some(n) => return n,
                None => {
                    eprintln!("[*] Queue 0 is empty.");
                    return 0;
                }
            }
        } else if memory_number == 27 {
            eprintln!("[*] Pipe is not implemented.");
            return 0;
        } else {
            eprintln!("[*] {} is invalid storage number.", memory_number);
            return 0;
        } 
    }
}

pub fn get_memory_number(memory_name: char) -> usize {
    match JONGSEONG_LIST.iter().position(|&c| c == memory_name) {
        Some(n) => return n,
        None => {
            eprintln!("[*] {} is invalid storage number.", memory_name);
            return 0;
        }
    }
}