pub mod hangul;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Nokheui {
    code: Vec<Vec<char>>,
    cursor: (u32, u32),
    stacks: [Vec<i32>; 26],
    queue: VecDeque<i32>
}

impl Nokheui {
    pub fn new(code: Vec<Vec<char>>) -> Nokheui {
        Nokheui {
            code: code,
            cursor: (0, 0),
            stacks: Default::default(),
            queue: Default::default(),
        }
    }

    pub fn run(&self) -> i32 {
        return 0;
    }
}