use logos::{Logos, Lexer, Skip};

#[derive(Debug, Logos)]
#[logos(extras = (usize, usize))]
#[logos(skip r"[\t\f ]*")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex(r"\n", newline_callback)]
    Newline,

    #[regex(r"//[^\n]*", newline_callback)]
    Commentary,

    //////////////
    //Identifier//
    //////////////
    #[regex(r"[a-zA-Z][0-9a-zA-Z]*", word_callback)]
    Identifier((usize, usize,)),

    ////////////
    //Operators/
    ////////////
    #[token("*", word_callback)]
    OperatorMult((usize, usize)),

    #[token("=", word_callback)]
    OperatorEqual((usize, usize)),

    #[token("+", word_callback)]
    OperatorPlus((usize, usize)),

    #[token("/", word_callback)]
    OperatorDiv((usize, usize)),
    
    #[token("-", word_callback)]
    OperatorMinus((usize, usize)),

    #[token(">", word_callback)]
    OperatorGreater((usize, usize)),

    #[token(">=", word_callback)]
    OperatorGreaterEq((usize, usize)),

    #[token("<", word_callback)]
    OperatorLess((usize, usize)),

    #[token("<=", word_callback)]
    OperatorLesseq((usize, usize)),

    #[token("%", word_callback)]
    OperatorMod((usize, usize)),

    /////////////
    //Constants//
    /////////////
    
    #[regex(r"(?:0|[1-9][_?0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?", word_callback)]
    ConstantNumeric((usize, usize)),

    
    #[regex(r"\'[a-zA-Z]\'", word_callback)]
    ConstantChar((usize, usize)),

    ///////////
    //Keywords/
    ///////////


    #[token("func", word_callback)]
    KeywordFunc((usize, usize)),

    #[token("main", word_callback)]
    KeywordMain((usize, usize)),

    #[token("if", word_callback)]
    KeywordIf((usize, usize)),

    #[token("else if", word_callback)]
    KeywordElseIf((usize, usize)),

    #[token("else", word_callback)]
    KeywordElse((usize, usize)),

    #[token("let", word_callback)]
    KeywordLet((usize, usize)),

    #[token("false", word_callback)]
    KeywordFalse((usize, usize)),

    #[token("true", word_callback)]
    KeywordTrue((usize, usize)),

    #[token("while", word_callback)]
    KeywordWhile((usize, usize)),

    ////////////
    //Literals//
    ////////////

    #[regex("\"(?s:[^\"\\\\]|\\\\.)*\"", word_callback)]
    StringLiteral((usize, usize)),

    ////////////////
    //Punctuations//
    ////////////////
    #[token("[", word_callback)]
    PunctuationLBracket((usize, usize)),

    #[token("]", word_callback)]
    PunctuationRBracket((usize, usize)),

    #[token(";", word_callback)]
    PunctuationSemiColon((usize, usize)),

    #[token("(", word_callback)]
    PunctuationLParent((usize, usize)),

    #[token(")", word_callback)]
    PunctuationRParent((usize, usize)),

    #[token("{", word_callback)]
    PunctuationLCurly((usize, usize)),

    #[token("}", word_callback)]
    PunctuationRCurly((usize, usize)),

    #[token(",", word_callback)]
    PunctuationComma((usize, usize)),


    /////////////////
    //Special Chars//
    /////////////////

    #[token(":", word_callback)]
    SpecialCharColon((usize, usize)),

    #[token("->", word_callback)]
    SpecialCharArrow((usize, usize)),

}

fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    Skip
}

// Compute the line and column position for the current word.
fn word_callback(lex: &mut Lexer<Token>) -> (usize, usize) {
    let line = lex.extras.0;
    let column = lex.span().start - lex.extras.1;
    (line, column)
}

