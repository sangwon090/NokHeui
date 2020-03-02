pub mod hangul;

use std::io;
use std::collections::VecDeque;
use crate::hangul::*;

#[derive(Debug)]
pub struct Nokheui {
    code: Vec<Vec<char>>,
    cursor: (usize, usize),
    stacks: [Vec<i32>; 26],
    queue: VecDeque<i32>,
    selected_data: usize
}

impl Nokheui {
    pub fn new(code: Vec<Vec<char>>) -> Nokheui {
        Nokheui {
            code: code,
            cursor: (0, 0),
            stacks: Default::default(),
            queue: Default::default(),
            selected_data: 0
        }
    }

    pub fn run(&mut self) -> i32 {
        let current_char = self.code[self.cursor.0][self.cursor.1];

        if is_hangul(current_char) {
            let jaso = disassemble_hangul(current_char).unwrap();

            match jaso.0 {
                'ㅎ' => {
                    match self.get(self.selected_data) {
                        Some(n) => return n,
                        None => return 0
                    }
                },
                'ㄷ' => {
                    let a: i32 = self.get(self.selected_data).unwrap();
                    let b: i32 = self.get(self.selected_data).unwrap();

                    self.put(self.selected_data, a + b);
                },
                'ㄸ' => {
                    let a: i32 = self.get(self.selected_data).unwrap();
                    let b: i32 = self.get(self.selected_data).unwrap();

                    self.put(self.selected_data, a * b);
                },
                'ㅌ' => {
                    let a: i32 = self.get(self.selected_data).unwrap();
                    let b: i32 = self.get(self.selected_data).unwrap();

                    self.put(self.selected_data, a - b);
                },
                'ㄴ' => {
                    let a: i32 = self.get(self.selected_data).unwrap();
                    let b: i32 = self.get(self.selected_data).unwrap();

                    self.put(self.selected_data, a / b);
                },
                'ㄹ' => {
                    let a: i32 = self.get(self.selected_data).unwrap();
                    let b: i32 = self.get(self.selected_data).unwrap();

                    self.put(self.selected_data, a % b);
                }
                'ㅁ' => {
                    let value: i32 = self.get(self.selected_data).unwrap();

                    match jaso.2 {
                        'ㅇ' => {
                            print!("{}", value)
                        },
                        'ㅎ' => {
                            print!("{}", std::char::from_u32(value as u32).unwrap());
                        },
                        _ => {

                        }
                    }
                },
                'ㅂ' => {

                },
                'ㅃ' => {

                },
                'ㅍ' => {

                },
                'ㅅ' => {
                    self.selected_data = get_jongseong_position(jaso.2);
                },
                'ㅆ' => {
                    let value: i32 = self.get(self.selected_data).unwrap();
                    self.put(get_jongseong_position(jaso.2), value);
                },
                'ㅈ' => {
                    let a: i32 = self.get(self.selected_data).unwrap();
                    let b: i32 = self.get(self.selected_data).unwrap();

                    self.put(self.selected_data, if a <= b{
                        1
                    } else {
                        2
                    });
                },
                'ㅊ' => {
                    
                },
                _ => {

                }
            }
        } else {
            
        }

        return 0;
    }

    fn put(&mut self, selected_data: usize, value: i32) {
        if selected_data < 26 {
            self.stacks[selected_data].push(value);
        } else if selected_data == 26 {
            return self.queue.push_back(value);
        }
    }

    fn get(&mut self, selected_data: usize) -> Option<i32> {
        if selected_data < 26 {
            return self.stacks[selected_data].pop();
        } else if selected_data == 26 {
            return self.queue.pop_front();
        } else {
            return None
        }
    }
}