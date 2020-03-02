pub mod hangul;
pub mod storage;

use crate::hangul::*;
use crate::storage::Storage;

pub struct Nokheui {
    code: Vec<Vec<char>>,
    cursor: (usize, usize),
    storage: Storage,
    selected_data: usize
}

impl Nokheui {
    pub fn new(code: Vec<Vec<char>>) -> Nokheui {
        Nokheui {
            code: code,
            cursor: (0, 0),
            storage: Storage::new(),
            selected_data: 0
        }
    }

    pub fn run(&mut self) -> i32 {
        let current_char = self.code[self.cursor.0][self.cursor.1];

        if is_hangul(current_char) {
            let jaso = disassemble_hangul(current_char).unwrap();

            match jaso.0 {
                'ㅎ' => {
                    return self.storage.pop(self.selected_data);
                },
                'ㄷ' => {
                    let a: i32 = self.storage.pop(self.selected_data);
                    let b: i32 = self.storage.pop(self.selected_data);

                    self.storage.push(self.selected_data, a + b);
                },
                'ㄸ' => {
                    let a: i32 = self.storage.pop(self.selected_data);
                    let b: i32 = self.storage.pop(self.selected_data);

                    self.storage.push(self.selected_data, a * b);
                },
                'ㅌ' => {
                    let a: i32 = self.storage.pop(self.selected_data);
                    let b: i32 = self.storage.pop(self.selected_data);

                    self.storage.push(self.selected_data, a - b);
                },
                'ㄴ' => {
                    let a: i32 = self.storage.pop(self.selected_data);
                    let b: i32 = self.storage.pop(self.selected_data);

                    self.storage.push(self.selected_data, a / b);
                },
                'ㄹ' => {
                    let a: i32 = self.storage.pop(self.selected_data);
                    let b: i32 = self.storage.pop(self.selected_data);

                    self.storage.push(self.selected_data, a % b);
                }
                'ㅁ' => {
                    let value: i32 = self.storage.pop(self.selected_data);

                    match jaso.2 {
                        'ㅇ' => {
                            print!("{}", value)
                        },
                        'ㅎ' => {
                            print!("{}", std::char::from_u32(value as u32).unwrap());
                        },
                        _ => {
                            eprintln!("[*] Jongseong '{}' is invalid for output format.", jaso.2)
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
                    self.selected_data = storage::get_memory_number(jaso.2);
                },
                'ㅆ' => {
                    let value: i32 = self.storage.pop(self.selected_data);

                    self.storage.push(storage::get_memory_number(jaso.2), value);
                },
                'ㅈ' => {
                    let a: i32 = self.storage.pop(self.selected_data);
                    let b: i32 = self.storage.pop(self.selected_data);

                    self.storage.push(self.selected_data, if a <= b{
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
}