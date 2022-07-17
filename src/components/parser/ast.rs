use super::operations::Func;
use super::parser::{ParseError, Parser};
use super::token::Token;
use std::fmt::{self, Debug};

pub enum Expression {
    Ident(Box<str>),
    Number(f64),
    List(Vec<Expression>),
    Op(Box<Func>),
}

use Expression::*;

impl Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = match self {
            Ident(s) => format!("{s:#?}"),
            Number(f) => format!("{f:#?}"),
            List(vec) => format!("{vec:#?}"),
            Op(func) => format!("Risp function object: {}", func.name),
        };
        write!(f, "{fmt}")
    }
}

#[derive(Debug, Default)]
pub struct Ast {
    inner: Box<[Expression]>,
    tokens: Box<[Token]>,
    cursor: usize,
    peek: usize,
}
impl Ast {
    pub fn new() -> Self {
        Ast::default()
    }

    pub fn emit(&mut self, parser: Parser, list: bool) -> Result<(), ParseError> {
        self.tokens = parser.parsed;

        let mut expressions = Vec::new();
        expressions.reserve(1 << 8);
        while let Some(token) = self.next().map(|token| token.clone()) {
            if expressions.capacity() == 0 {
                expressions.reserve(1 << 8);
            }
            let expression = self.match_grammar(&token)?;
            expressions.push(expression);
        }
        Ok(())
    }

    fn match_grammar(&mut self, token: &Token) -> Result<Expression, ParseError> {
        unimplemented!()
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.cursor);
        self.cursor += 1;
        token
    }

    fn peek(&mut self) -> Option<&Token> {
        self.peek += 1;
        self.tokens.get(self.peek)
    }

    // Assumes that:
    //  - (self.cursor - self.peek) will be a valid index in self.tokens
    //  - you actually checked all values return Some(_) before calling
    // Pushes all tokens up to one before self.peek
    unsafe fn next_to_peek(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        for _ in 0..self.peek {
            let token = self.next().map(|token| token.clone()).unwrap();
            tokens.push(token);
        }
        self.peek = 0;
        tokens
    }
}
