use std::collections::VecDeque;

use crate::hangul::{JONGSEONG_LIST};

pub enum StorageType {
    Stack(usize),
    Queue,
    Pipe
}

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
                        eprintln!("[*] Queue 0 is empty.");
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
                let value: i32 = self.stacks[*n].last().cloned().unwrap();

                self.stacks[*n].push(value);   
            },
            StorageType::Queue => {
                let value: i32 = self.queue[0];

                self.queue.push_front(value);
            },
            StorageType::Pipe => {
                eprintln!("[*] Pipe is not implemented.")
            }
        }
    }

    pub fn swap(&mut self, storage_type: &StorageType) {
        match storage_type {
            StorageType::Stack(n) => {
                let length: usize = self.stacks[*n].len();

                self.stacks[*n].swap(length - 1, length - 2);
            },
            StorageType::Queue => {
                let a: i32 = match self.queue.pop_front() {
                    Some(n) => n,
                    None => {
                        eprintln!("[*] Queue 0 is empty.");
                        0
                    }
                };
    
                let b: i32 = match self.queue.pop_front() {
                    Some(n) => n,
                    None => {
                        eprintln!("[*] Queue 0 is empty.");
                        0
                    }
                };
    
                self.queue.push_front(a);
                self.queue.push_front(b);
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
        0..=7 => {
            return Some(StorageType::Stack(storage_number));
        },
        8 => {
            return Some(StorageType::Queue);
        },
        9..=13 => {
            return Some(StorageType::Stack(storage_number - 1));
        },
        14 => {
            return Some(StorageType::Pipe);
        },
        15..=27 => {
            return Some(StorageType::Stack(storage_number - 2));
        },
        _ => {
            eprintln!("[*] {} is invalid storage number.", storage_number);
            return None;
        }
    }
}