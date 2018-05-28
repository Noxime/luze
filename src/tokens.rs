use std::fmt::Debug;

#[derive(Debug)]
pub struct Position {
    pos: usize,
    len: usize,
}

pub trait Token: Debug {
    fn pos(&self) -> Position {
        Position {
            pos: 0,
            len: 0,
        }
    }
}

pub fn tokenize(src: &str) -> Vec<Box<Token>> {
    vec![]
}