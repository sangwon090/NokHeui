pub mod hangul;
pub mod storage;

use std::io;

use crate::hangul::*;
use crate::storage::*;

pub struct Nokheui {
    code: Vec<Vec<char>>,
    cursor: (usize, usize),
    velocity: (i32, i32),
    storage: Storage,
    selected_storage: StorageType
}

impl Nokheui {
    pub fn new(code: Vec<Vec<char>>) -> Nokheui {
        Nokheui {
            code: code,
            cursor: (0, 0),
            velocity: (0, 1),
            storage: Storage::new(),
            selected_storage: StorageType::Stack(0)
        }
    }

    pub fn run(&mut self) -> i32 {
        loop {
            let current_char: char = self.code[self.cursor.1][self.cursor.0];
            let mut is_redirected: bool = false;

            if is_hangul(current_char) {
                let jaso = disassemble_hangul(current_char).unwrap();

                match jaso.0 {
                    'ㅎ' => {
                        break;
                    },
                    'ㄷ' => {
                        let a: i32 = self.storage.pop(&self.selected_storage);
                        let b: i32 = self.storage.pop(&self.selected_storage);

                        self.storage.push(&self.selected_storage, b + a);
                    },
                    'ㄸ' => {
                        let a: i32 = self.storage.pop(&self.selected_storage);
                        let b: i32 = self.storage.pop(&self.selected_storage);

                        self.storage.push(&self.selected_storage, b * a);
                    },
                    'ㅌ' => {
                        let a: i32 = self.storage.pop(&self.selected_storage);
                        let b: i32 = self.storage.pop(&self.selected_storage);

                        self.storage.push(&self.selected_storage, b - a);
                    },
                    'ㄴ' => {
                        let a: i32 = self.storage.pop(&self.selected_storage);
                        let b: i32 = self.storage.pop(&self.selected_storage);

                        self.storage.push(&self.selected_storage, b / a);
                    },
                    'ㄹ' => {
                        let a: i32 = self.storage.pop(&self.selected_storage);
                        let b: i32 = self.storage.pop(&self.selected_storage);

                        self.storage.push(&self.selected_storage, b % a);
                    }
                    'ㅁ' => {
                        let value: i32 = self.storage.pop(&self.selected_storage);

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
                        match jaso.2 {
                            'ㅇ' => {
                                let mut input: String = String::new();

                                match io::stdin().read_line(&mut input) {
                                    Ok(_) => {
                                        match input.trim().parse::<i32>() {
                                            Ok(n) => {

                                                self.storage.push(&self.selected_storage, n);
                                            },
                                            Err(_) => {
                                                eprintln!("[*] {} is invalid integer.", input);
                                                self.storage.push(&self.selected_storage, 0);
                                            }
                                        }
                                    },
                                    Err(_) => {
                                        eprintln!("[*] Cannot read a line from stdin.");
                                        self.storage.push(&self.selected_storage, 0);
                                    }
                                };
                            },
                            'ㅎ' => {
                                let mut input: String = String::new();
                                self.storage.push(&self.selected_storage, match io::stdin().read_line(&mut input) {
                                    Ok(_) => {
                                        input.chars().next().unwrap() as i32
                                    },
                                    Err(_) => {
                                        eprintln!("[*] Cannot read a line from stdin.");
                                        0
                                    }
                                });
                            },
                            'ㄱ' | 'ㄴ' | 'ㅅ' => {
                                self.storage.push(&self.selected_storage, 2);
                            },
                            'ㄷ' | 'ㅈ' | 'ㅋ' => {
                                self.storage.push(&self.selected_storage, 3);
                            },
                            'ㅁ' | 'ㅂ' | 'ㅊ' | 'ㅌ' | 'ㅍ' | 'ㄲ' | 'ㄳ' | 'ㅆ' => {
                                self.storage.push(&self.selected_storage, 4);
                            },
                            'ㄹ' | 'ㄵ'	| 'ㄶ' => {
                                self.storage.push(&self.selected_storage, 5);
                            },
                            'ㅄ' => {
                                self.storage.push(&self.selected_storage, 6);
                            },
                            'ㄺ' | 'ㄽ' => {
                                self.storage.push(&self.selected_storage, 7);
                            },
                            'ㅀ' => {
                                self.storage.push(&self.selected_storage, 8);
                            },
                            'ㄻ' | 'ㄼ' | 'ㄾ' | 'ㄿ' => {
                                self.storage.push(&self.selected_storage, 9);
                            },
                            _ => {
                                self.storage.push(&self.selected_storage, 0);
                            }
                        }
                    },
                    'ㅃ' => {
                        self.storage.duplicate(&self.selected_storage);
                    },
                    'ㅍ' => {
                        self.storage.swap(&self.selected_storage);
                    },
                    'ㅅ' => {
                        self.selected_storage = match storage::get_storage(storage::get_storage_number(jaso.2)) {
                            Some(s) => s,
                            None => {
                                eprintln!("[*] {} is invalid storage name.", jaso.2);
                                StorageType::Stack(0)
                            }
                        }
                    },
                    'ㅆ' => {
                        let value: i32 = self.storage.pop(&self.selected_storage);
                        let storage: StorageType = match storage::get_storage(storage::get_storage_number(jaso.2)) {
                            Some(s) => s,
                            None => {
                                eprintln!("[*] {} is invalid storage name.", jaso.2);
                                StorageType::Stack(0)
                            }
                        };

                        self.storage.push(&storage, value);
                    },
                    'ㅈ' => {
                        let a: i32 = self.storage.pop(&self.selected_storage);
                        let b: i32 = self.storage.pop(&self.selected_storage);

                        self.storage.push(&self.selected_storage, if a <= b{
                            1
                        } else {
                            2
                        });
                    },
                    'ㅊ' => {
                        let value: i32 = self.storage.pop(&self.selected_storage);

                        if value == 0 {
                            is_redirected = true;
                        }
                    },
                    _ => {

                    }
                }

                match jaso.1 {
                    'ㅏ' => {
                        self.velocity = (1, 0);
                    },
                    'ㅓ' => {
                        self.velocity = (-1, 0);
                    },
                    'ㅗ' => {
                        self.velocity = (0, -1);
                    },
                    'ㅜ' => {
                        self.velocity = (0, 1);
                    },
                    'ㅑ' => {
                        self.velocity = (2, 0);
                    },
                    'ㅕ' => {
                        self.velocity = (-2, 0);
                    },
                    'ㅛ' => {
                        self.velocity = (0, -2);
                    },
                    'ㅠ' => {
                        self.velocity = (0, 2);
                    },
                    'ㅡ' => {
                        if self.velocity.0 == 0 && self.velocity.1 != 0 {
                            self.velocity = (-self.velocity.0, -self.velocity.1);
                        }
                    },
                    'ㅣ' => {
                        if self.velocity.0 != 0 && self.velocity.1 == 0 {
                            self.velocity = (-self.velocity.0, -self.velocity.1);
                        }
                    },
                    'ㅢ' => {
                        self.velocity = (-self.velocity.0, -self.velocity.1);
                    },
                    _ => {

                    }
                }

                if is_redirected {
                    self.velocity = (-self.velocity.0, -self.velocity.1);
                }
            }

            self.move_cursor(self.velocity);
        }
        
        return self.storage.pop(&self.selected_storage);
    }

    fn move_cursor(&mut self, velocity: (i32, i32)) {
        let new_x: i32 = (self.cursor.0 as i32) + velocity.0;
        let new_y: i32 = (self.cursor.1 as i32) + velocity.1;

        if new_x >= (self.code[self.cursor.1].len() as i32) {
            self.cursor.0 = 0;
        } else if new_x < 0 {
            self.cursor.0 = self.code[self.cursor.1].len() - 1;
        } else {
            self.cursor.0 = new_x as usize;
        }

        if new_y >= (self.code.len() as i32) {
            for i in 0..self.code.len() {
                if new_x < (self.code[i].len() as i32) {
                    self.cursor.1 = i;
                    break;
                }
            }
        } else if new_y < 0 {
            for i in (0..self.code.len()).rev() {
                if new_x < (self.code[i].len() as i32) {
                    self.cursor.1 = i;
                    break;
                }
            }
        } else {
            if velocity.1 > 0 && new_x >= (self.code[new_y as usize].len() as i32) {
                for i in (new_y as usize)..self.code.len() {
                    if new_x < (self.code[i].len() as i32) {
                        self.cursor.1 = i;
                        break;
                    }
                }
            } else if velocity.1 < 0 && new_x >= (self.code[new_y as usize].len() as i32) {
                for i in ((new_y as usize)..self.code.len()).rev() {
                    if new_x < (self.code[i].len() as i32) {
                        self.cursor.1 = i;
                        break;
                    }
                }
            } else {
                self.cursor.1 = new_y as usize;
            }
        }
    }
}