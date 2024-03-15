use logos::{Logos, Lexer, Skip};

#[derive(Debug, Logos)]
#[logos(extras = (usize, usize))]
#[logos(error = LexingError)]
#[logos(skip r"[\t\f ]*")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex(r"\n", priority = 10, callback = newline_callback)]
    Newline,

    #[regex(r"//[^\n]*", priority = 10, callback = newline_callback)]
    Commentary,

    #[regex(r"[a-zA-Z][0-9a-zA-Z]*", priority = 10, callback = word_callback)]
    Identifier((usize, usize,)),

    #[regex(r"([\*=\+/\-><%]|(>=)|(<=))", priority = 10, callback = word_callback)]
    Operator((usize, usize)),
    
    #[regex(r"(?:0|\-?[1-9][_?0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?", word_callback)]
    ConstantNumeric((usize, usize)),
    
    #[regex(r"\'[a-zA-Z]\'", priority = 10, callback = word_callback)]
    ConstantChar((usize, usize)),

    #[regex(r"(func|if|let|else (if)?|false|true|while|int|float|bool|char|String|return)", priority = 15, callback = word_callback)]
    Keyword((usize, usize)),

    #[regex("\"(?s:[^\"\\\\]|\\\\.)*\"", priority = 10, callback = word_callback)]
    StringLiteral((usize, usize)),

    #[regex(r"[\]\[;)(}{,:]", priority = 10, callback = word_callback)]
    Punctuation((usize, usize)),

    #[token("->", priority = 10, callback = word_callback)]
    SpecialChar((usize, usize)),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    #[default]
    UnexpectedToken,
}


fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    Skip
}

fn word_callback(lex: &mut Lexer<Token>) -> (usize, usize) {
    let line = lex.extras.0;
    let column = lex.span().start - lex.extras.1;
    (line, column)
}

