mod operations;

use operations::ConstOp;
use std::fmt::{self, Debug};

pub enum Expression {
    Ident(Box<str>),
    Number(f64),
    List(Vec<Expression>),
    Op(ConstOp),
}
use Expression::*;

impl Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = match self {
            Ident(s) => format!("{s:#?}"),
            Number(f) => format!("{f:#?}"),
            List(vec) => format!("{vec:#?}"),
            Op((name, _)) => format!("Risp function object: {name}"),
        };
        write!(f, "{fmt}")
    }
}

pub fn match_grammar() {}
