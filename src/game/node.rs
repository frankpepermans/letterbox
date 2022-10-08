use std::ops::{Index, IndexMut};

pub enum Entry {
    LEFT,
    TOP,
    RIGHT,
    BOTTOM,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub left: bool,
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
}

impl Node {
    pub fn open() -> Self {
        Self {
            left: true,
            top: true,
            right: true,
            bottom: true,
        }
    }

    pub fn closed() -> Self {
        Self {
            left: false,
            top: false,
            right: false,
            bottom: false,
        }
    }
}

impl From<u8> for Node {
    fn from(value: u8) -> Self {
        Self {
            left: value & 0b1000 == 0b1000,
            top: value & 0b100 == 0b100,
            right: value & 0b10 == 0b10,
            bottom: value & 0b1 == 0b1,
        }
    }
}

impl Into<u8> for Node {
    fn into(self) -> u8 {
        let l = if self.left { 0b1000 } else { 0b0 };
        let t = if self.top { 0b100 } else { 0b0 };
        let r = if self.right { 0b10 } else { 0b0 };
        let b = if self.bottom { 0b1 } else { 0b0 };

        l | t | r | b
    }
}

impl Index<Entry> for Node {
    type Output = bool;

    fn index(&self, index: Entry) -> &Self::Output {
        match index {
            Entry::LEFT => &self.left,
            Entry::TOP => &self.top,
            Entry::RIGHT => &self.right,
            Entry::BOTTOM => &self.bottom,
        }
    }
}

impl IndexMut<Entry> for Node {
    fn index_mut(&mut self, index: Entry) -> &mut Self::Output {
        match index {
            Entry::LEFT => &mut self.left,
            Entry::TOP => &mut self.top,
            Entry::RIGHT => &mut self.right,
            Entry::BOTTOM => &mut self.bottom,
        }
    }
}
