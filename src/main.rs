use logos::Logos;
use std::env;
use std::fs;
use std::io::Write;
use std::fs::File;

mod tokens;
use tokens::{Token, LexingError};


fn main() -> std::io::Result<()> {

    let file = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file");

    let mut lex = Token::lexer(file.as_str());
    let mut token_cont = 0;

    let mut write_file = File::create("Tokens.txt")?;

    while let Some(token) = lex.next() {
        match token{
            Ok(Token::Keyword((line, column))) =>
                writeln!(write_file, "Keyword '{}' found at ({}, {})\n"
                            ,lex.slice(), line+1, column+1)?,

            Ok(Token::Identifier((line, column))) =>
                writeln!(write_file, "Identifier '{}' found at ({}, {})\n"
                            ,lex.slice(), line+1, column+1)?,

            Ok(Token::Operator((line, column))) =>
                writeln!(write_file, "Operator '{}' found at ({}, {})\n",
                            lex.slice(), line+1, column+1)?,

            Ok(Token::ConstantNumeric((line, column))) =>
                writeln!(write_file, "ConstantNumeric '{}' found at ({}, {})\n",
                            lex.slice(), line+1, column+1)?,

            Ok(Token::ConstantChar((line, column))) =>
                writeln!(write_file, "ConstantChar '{}' found at ({}, {})\n",
                            lex.slice(), line+1, column+1)?,

            Ok(Token::StringLiteral((line, column))) =>
                writeln!(write_file, "StringLiteral '{}' found at ({}, {})\n",
                            lex.slice(), line+1, column+1)?,

            Ok(Token::Punctuation((line, column))) =>
                writeln!(write_file, "Punctuation '{}' found at ({}, {})\n",
                            lex.slice(), line+1, column+1)?,

            Ok(Token::SpecialChar((line, column))) =>
                writeln!(write_file, "SpecialChar '{}' found at ({}, {})\n",
                            lex.slice(), line+1, column+1)?,

            Err(LexingError::UnexpectedToken) => {
                let (line, col) = find_line_and_column(file.as_str(), lex.span());
                panic!("Error! found unexpected token: '{}' at ({line}, {col})", lex.slice());
                },
            _ => panic!("Unknown error! {:#?}", lex.slice()),

        }
        token_cont+=1;
    }

    println!("\nLexing process succeded! :D \nTotal tokens found: {token_cont}");
    Ok(())
}

fn find_line_and_column(file: &str, span: logos::Span) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;

    for (i, c) in file.chars().enumerate() {
        if i >= span.start {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}



