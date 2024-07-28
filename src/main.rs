use std::fs;
use std::env;


enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    // Mod?
}

enum Identifier {
    Fraction,
    Summation,
}

enum Symbol {
    Superscript,
    Subscript,
}

enum Group {
    Group,
    Parenthesis,
    Bracket,
    Brace,
    Angle,
}

enum Token {
    BinaryOperator(BinaryOperator),
    Variable(String),
    Begin(Group),
    End(Group),
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let path = &args[1];

    let contents = fs::read_to_string(path).expect("Should have been able to read file");

    let token = Token{};


    println!("{contents}")
}
