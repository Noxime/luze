use std::fmt::Debug;

#[allow(unused)]
#[derive(Debug)]
pub enum Token {
    BraceOpen,  // {
    BraceClose, // }
    ParenOpen,  // (
    ParenClose, // )
    BlockOpen,  // [
    BlockClose, // ]
    
    Semi,    // ;
    Equal,   // =

    KwIs,   // `is`
    KwHas,  // `has`
    KwFits, // `fits`

    ConstInt(usize), // eg 231 
    // there are no negative values in src files as -x is function
    ConstStr(String),  // eg "Hello world"
    ConstBool(bool),  // eg true
    Identifier(String) // eg foo
}

fn split(s: &str) -> Vec<String> {
    let mut res = vec![];

    let mut quotes = false;
    let mut buf = String::new();
    for c in s.chars() {
        if quotes {
            if c == '"' {
                quotes = false;
            }
            buf.push(c);
            continue;
        } else {
            if c == '"' {
                quotes = true;
            }
        }

        if false
            | (c == '{')
            | (c == '}')
            | (c == '(')
            | (c == ')')
            | (c == '[')
            | (c == ']')
            | (c == ';')
            | (c == '=')
        {
            buf = buf.trim().to_string();
            if buf.len() != 0 {
                res.push(buf);
            }
            res.push(c.to_string());
            buf = String::new();
            continue;
        } 

        if c.is_whitespace() {
            buf = buf.trim().to_string();
            if buf.len() != 0 {
                res.push(buf);
                buf = String::new();
            }
        }

        buf.push(c);
    }
    
    res
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut res = vec![];

    for item in split(src) {
        res.push(match item.as_str() {
            "{" => Token::BraceOpen,
            "}" => Token::BraceClose,
            "(" => Token::ParenOpen,
            ")" => Token::ParenClose,
            "[" => Token::BlockOpen,
            "]" => Token::BlockClose,

            ";" => Token::Semi,
            "=" => Token::Equal,

            "is" => Token::KwIs,
            "has" => Token::KwHas,
            "fits" => Token::KwFits,
            
            v => {
                if v.chars().next() == Some('"') && v.chars().last() == Some('"') {
                    let mut s = v.to_string().split_off(1);
                    s.truncate(v.len() - 2);
                    Token::ConstStr(s)
                } else if let Ok(v) = v.parse::<usize>() {
                    Token::ConstInt(v)
                } else if let Ok(v) = v.parse::<bool>() {
                    Token::ConstBool(v)
                } else {
                    Token::Identifier(v.to_string())
                }
            },
        });
    }

    res
}