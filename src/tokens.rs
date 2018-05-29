use std::fmt::Debug;

#[allow(unused)]
#[derive(Debug)]
pub enum Token {
    BraceOpen,  // {
    BraceClose, // }
    ParenOpen,  // (
    ParenClose, // )
    
    Semi,    // ;
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Equal,   // =
    Greater, // >
    Less,    // <
    Not,     // !
    Period,  // .

    KwStruct, // "struct"
    KwEnum,   // "enum"
    KwHas,    // "has"
    KwIs,     // "is"
    KwPure,   // "pure"
    KwImpure, // "impure"
    KwTask,   // "task"
    KwIf,     // "if"
    KwElse,   // "else"
    KwElif,   // "elif"
    KwMatch,  // "match"

}

pub fn tokenize(src: &str) -> Vec<Box<Token>> {
    vec![]
}