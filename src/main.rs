use logos::Logos;
use std::env;
use std::fs;

mod tokens;
use tokens::Token;



fn main(){
let file = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let mut lex = Token::lexer(file.as_str()); //Esta tiene que ser mut

    
    for result in Token::lexer(file.as_str()) {
        match result {
            Ok(token) => print!("{:?}", token),
            Err(e) => panic!("some error occurred: {:?}", e),
        }
        lex.next();
        println!("\n------> {}", lex.slice());
    }
}


