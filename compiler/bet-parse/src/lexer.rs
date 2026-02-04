// SPDX-License-Identifier: MIT OR Apache-2.0
//! Lexer for Betlang using logos

use logos::Logos;
use bet_syntax::Span;

/// Token types for betlang
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]  // Skip whitespace
#[logos(skip r"--[^\n]*")]       // Skip line comments
#[logos(skip r"\{-([^-]|-[^\}])*-\}")]  // Skip block comments
pub enum Token {
    // === Keywords ===
    #[token("bet")]
    Bet,

    #[token("let")]
    Let,

    #[token("in")]
    In,

    #[token("fun")]
    Fun,

    #[token("match")]
    Match,

    #[token("if")]
    If,

    #[token("then")]
    Then,

    #[token("else")]
    Else,

    #[token("end")]
    End,

    #[token("do")]
    Do,

    #[token("return")]
    Return,

    #[token("type")]
    Type,

    #[token("module")]
    Module,

    #[token("import")]
    Import,

    #[token("export")]
    Export,

    #[token("rec")]
    Rec,

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("not")]
    Not,

    #[token("xor")]
    Xor,

    // === Ternary-specific ===
    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("unknown")]
    Unknown,

    // === Probabilistic keywords ===
    #[token("sample")]
    Sample,

    #[token("observe")]
    Observe,

    #[token("infer")]
    Infer,

    #[token("parallel")]
    Parallel,

    #[token("weighted")]
    Weighted,

    // === Inference methods ===
    #[token("MCMC")]
    MCMC,

    #[token("HMC")]
    HMC,

    #[token("SMC")]
    SMC,

    #[token("VI")]
    VI,

    // === Literals ===
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Int(i64),

    #[regex(r"[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>().ok())]
    #[regex(r"[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        Some(s[1..s.len()-1].to_string())
    })]
    String(String),

    // === Identifiers ===
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"'[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice()[1..].to_string())]
    TypeVar(String),

    // === Operators ===
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("^")]
    Caret,

    #[token("==")]
    EqEq,

    #[token("!=")]
    NotEq,

    #[token("<")]
    Lt,

    #[token("<=")]
    Le,

    #[token(">")]
    Gt,

    #[token(">=")]
    Ge,

    #[token("&&")]
    AndAnd,

    #[token("||")]
    OrOr,

    #[token("++")]
    PlusPlus,

    #[token("::")]
    ColonColon,

    #[token(">>")]
    GtGt,

    #[token("|>")]
    Pipe,

    #[token("<-")]
    LArrow,

    #[token("->")]
    RArrow,

    #[token("=>")]
    FatArrow,

    // === Delimiters ===
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    // === Punctuation ===
    #[token(",")]
    Comma,

    #[token(";")]
    Semi,

    #[token(":")]
    Colon,

    #[token(".")]
    Dot,

    #[token("=")]
    Eq,

    #[token("@")]
    At,

    #[token("|")]
    Bar,

    #[token("_")]
    Underscore,

    #[token("?")]
    Question,

    #[token("\\")]
    Backslash,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Bet => write!(f, "bet"),
            Token::Let => write!(f, "let"),
            Token::In => write!(f, "in"),
            Token::Fun => write!(f, "fun"),
            Token::Match => write!(f, "match"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            Token::End => write!(f, "end"),
            Token::Do => write!(f, "do"),
            Token::Return => write!(f, "return"),
            Token::Type => write!(f, "type"),
            Token::Module => write!(f, "module"),
            Token::Import => write!(f, "import"),
            Token::Export => write!(f, "export"),
            Token::Rec => write!(f, "rec"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::Not => write!(f, "not"),
            Token::Xor => write!(f, "xor"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Unknown => write!(f, "unknown"),
            Token::Sample => write!(f, "sample"),
            Token::Observe => write!(f, "observe"),
            Token::Infer => write!(f, "infer"),
            Token::Parallel => write!(f, "parallel"),
            Token::Weighted => write!(f, "weighted"),
            Token::MCMC => write!(f, "MCMC"),
            Token::HMC => write!(f, "HMC"),
            Token::SMC => write!(f, "SMC"),
            Token::VI => write!(f, "VI"),
            Token::Int(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Ident(s) => write!(f, "{}", s),
            Token::TypeVar(s) => write!(f, "'{}", s),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Caret => write!(f, "^"),
            Token::EqEq => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Lt => write!(f, "<"),
            Token::Le => write!(f, "<="),
            Token::Gt => write!(f, ">"),
            Token::Ge => write!(f, ">="),
            Token::AndAnd => write!(f, "&&"),
            Token::OrOr => write!(f, "||"),
            Token::PlusPlus => write!(f, "++"),
            Token::ColonColon => write!(f, "::"),
            Token::GtGt => write!(f, ">>"),
            Token::Pipe => write!(f, "|>"),
            Token::LArrow => write!(f, "<-"),
            Token::RArrow => write!(f, "->"),
            Token::FatArrow => write!(f, "=>"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Semi => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::Dot => write!(f, "."),
            Token::Eq => write!(f, "="),
            Token::At => write!(f, "@"),
            Token::Bar => write!(f, "|"),
            Token::Underscore => write!(f, "_"),
            Token::Question => write!(f, "?"),
            Token::Backslash => write!(f, "\\"),
        }
    }
}

/// A token with its span
#[derive(Debug, Clone)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}

/// Lexer error
#[derive(Debug, Clone, thiserror::Error)]
pub enum LexError {
    #[error("Invalid token at position {0}")]
    InvalidToken(usize),
}

/// Lex a source string into tokens
pub fn lex(source: &str) -> Result<Vec<SpannedToken>, LexError> {
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => {
                let span = lexer.span();
                tokens.push(SpannedToken {
                    token,
                    span: Span::new(span.start as u32, span.end as u32),
                });
            }
            Err(_) => {
                return Err(LexError::InvalidToken(lexer.span().start));
            }
        }
    }

    Ok(tokens)
}

/// Iterator adapter for LALRPOP
pub struct Lexer<'input> {
    source: &'input str,
    logos: logos::Lexer<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(source: &'input str) -> Self {
        Self {
            source,
            logos: Token::lexer(source),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token, usize), LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.logos.next().map(|result| {
            let span = self.logos.span();
            match result {
                Ok(token) => Ok((span.start, token, span.end)),
                Err(_) => Err(LexError::InvalidToken(span.start)),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_bet() {
        let tokens = lex("bet { 1, 2, 3 }").unwrap();
        assert_eq!(tokens[0].token, Token::Bet);
        assert_eq!(tokens[1].token, Token::LBrace);
        assert_eq!(tokens[2].token, Token::Int(1));
    }

    #[test]
    fn test_lex_weighted_bet() {
        let tokens = lex("bet { a @ 0.5, b @ 0.3, c @ 0.2 }").unwrap();
        assert!(tokens.iter().any(|t| matches!(t.token, Token::At)));
    }

    #[test]
    fn test_lex_ternary() {
        let tokens = lex("true false unknown").unwrap();
        assert_eq!(tokens[0].token, Token::True);
        assert_eq!(tokens[1].token, Token::False);
        assert_eq!(tokens[2].token, Token::Unknown);
    }

    #[test]
    fn test_lex_function() {
        let tokens = lex("fun x -> x + 1").unwrap();
        assert_eq!(tokens[0].token, Token::Fun);
        assert_eq!(tokens[2].token, Token::RArrow);
    }

    #[test]
    fn test_lex_do_notation() {
        let tokens = lex("do { x <- sample dist; return x }").unwrap();
        assert_eq!(tokens[0].token, Token::Do);
        assert!(tokens.iter().any(|t| matches!(t.token, Token::LArrow)));
    }

    #[test]
    fn test_lex_end_keyword() {
        let tokens = lex("if x then y else z end").unwrap();
        assert!(tokens.iter().any(|t| matches!(t.token, Token::End)));
    }
}
