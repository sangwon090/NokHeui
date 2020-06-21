use std::collections::VecDeque;

use crate::hangul::{JONGSEONG_LIST};

pub enum StorageType {
    Stack(usize),
    Queue,
    Pipe
}

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

    pub fn push(&mut self, storage_type: &StorageType, value: i32) {
        match storage_type {
            StorageType::Stack(n) => {
                self.stacks[*n].push(value);
            },
            StorageType::Queue => {
                self.queue.push_back(value);
            },
            StorageType::Pipe => {
                eprintln!("[*] Pipe is not implemented.")
            }
        }
    }

    pub fn pop(&mut self, storage_type: &StorageType) -> i32 {
        match storage_type {
            StorageType::Stack(n) => {
                match self.stacks[*n].pop() {
                    Some(n) => return n,
                    None => {
                        eprintln!("[*] Stack {} is empty.", n);
                        return 0;
                    }
                }
            },
            StorageType::Queue => {
                match self.queue.pop_front() {
                    Some(n) => return n,
                    None => {
                        eprintln!("[*] Queue 0 is empty.!pop");
                        return 0;
                    }
                }
            },
            StorageType::Pipe => {
                eprintln!("[*] Pipe is not implemented.");
                return 0;
            }
        }
    }

    pub fn duplicate(&mut self, storage_type: &StorageType) {
        match storage_type {
            StorageType::Stack(n) => {
                if self.stacks[*n].len() > 0 {
                    let value: i32 = self.stacks[*n].last().cloned().unwrap();

                    self.stacks[*n].push(value);
                } else {
                    eprintln!("[*] Stack {} is empty.", n)
                }
            },
            StorageType::Queue => {
                if self.queue.len() > 0 {
                    let value: i32 = self.queue[0];

                    self.queue.push_front(value);
                } else {
                    eprintln!("[*] Queue 0 is empty.!dup")
                }
            },
            StorageType::Pipe => {
                eprintln!("[*] Pipe is not implemented.")
            }
        }
    }

    pub fn swap(&mut self, storage_type: &StorageType) {
        match storage_type {
            StorageType::Stack(n) => {
                if self.stacks[*n].len() >= 2 {
                    let length: usize = self.stacks[*n].len();

                    self.stacks[*n].swap(length - 1, length - 2);
                } else {
                    eprintln!("[*] Stack is not big enough to swap values.");
                }
            },
            StorageType::Queue => {
                if self.queue.len() >= 2 {
                    let a: i32 = self.queue.pop_front().unwrap();
                    let b: i32 = self.queue.pop_front().unwrap();

                    self.queue.push_front(a);
                    self.queue.push_front(b);
                } else {
                    eprintln!("[*] Queue is not big enough to swap values.");
                }
            },
            StorageType::Pipe => {
                eprintln!("[*] Pipe is not implemented.")
            }
        }
    }
}

pub fn get_storage_number(storage_name: char) -> usize {
    match JONGSEONG_LIST.iter().position(|&c| c == storage_name) {
        Some(n) => return n,
        None => {
            eprintln!("[*] {} is invalid storage number.", storage_name);
            return 0;
        }
    }
}

pub fn get_storage(storage_number: usize) -> Option<StorageType> {
    match storage_number {
        0..=20 => {
            return Some(StorageType::Stack(storage_number));
        },
        21 => {
            return Some(StorageType::Queue);
        },
        22..=26 => {
            return Some(StorageType::Stack(storage_number - 1));
        },
        27 => {
            return Some(StorageType::Pipe);
        },
        _ => {
            eprintln!("[*] {} is invalid storage number.", storage_number);
            return None;
        }
    }
}