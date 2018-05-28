use std::fs::File;
use std::io::Read;

mod tokens;

fn main() {
    let mut source = String::new();
    let _ = File::open("example.luz")
        .unwrap()
        .read_to_string(&mut source)
        .unwrap();

    println!("{:#?}", tokens::tokenize(&source));
}
