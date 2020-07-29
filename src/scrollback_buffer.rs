use std::collections::VecDeque;

pub struct ScrollbackBuffer {
    size: usize,
    buffer: VecDeque<String>,
}

impl ScrollbackBuffer {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            buffer: VecDeque::with_capacity(size),
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.buffer.iter()
    }

    pub fn push_back(&mut self, line: String) {
        while self.len() >= self.size {
            self.buffer.pop_front();
        }
        self.buffer.push_back(line);
    }
}
