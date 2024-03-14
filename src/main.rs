use logos::Logos;
use std::env;
use std::fs;

mod tokens;
use tokens::Token;
use tokens::LexingError;



fn main(){
let file = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let mut lex = Token::lexer(file.as_str()); //Esta tiene que ser mut

    /*
    for result in Token::lexer(file.as_str()) {
        match result {
            Ok(token) => print!("{:?}", token),
            Err(e) => panic!("some error occurred: {:?}", e),
        }
        lex.next();
        println!("\n------> {}", lex.slice());
    }*/

    while let Some(token) = lex.next() {
        match token{
            Ok(Token::Keyword((line, column))) => println!("Keyword '{}' found at ({}, {})", lex.slice(), line+1, column+1),
            Ok(Token::Identifier((line, column))) => println!("Identifier '{}' found at ({}, {})", lex.slice(), line+1, column+1),
            Ok(Token::Operator((line, column))) => println!("Operator '{}' found at ({}, {})", lex.slice(), line+1, column+1),
            Ok(Token::ConstantNumeric((line, column))) => println!("ConstantNumeric '{}' found at ({}, {})", lex.slice(), line+1, column+1),
            Ok(Token::ConstantChar((line, column))) => println!("ConstantChar '{}' found at ({}, {})", lex.slice(), line+1, column+1),
            Ok(Token::StringLiteral((line, column))) => println!("StringLiteral '{}' found at ({}, {})", lex.slice(), line+1, column+1),
            Ok(Token::Punctuation((line, column))) => println!("Punctuation '{}' found at ({}, {})", lex.slice(), line+1, column+1),
            Ok(Token::SpecialChar((line, column))) => println!("SpecialChar '{}' found at ({}, {})", lex.slice(), line+1, column+1),
            Err(LexingError::UnexpectedToken) => panic!("Error! unexpected token: {}", lex.slice()),
            _ => println!("No funca"),

        }
    }
}


