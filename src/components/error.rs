use std::fmt::{Display, Formatter};

use super::interpreter::operations::EvalError;
use super::interpreter::parser::ParseError;

#[derive(Debug)]
pub enum ErrorKind {
    Other,
    EvalError,
    ParseError,
}

pub struct RispError {
    kind: ErrorKind,
    msg: Box<str>,
}

impl Display for RispError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RispError: {:#?}: {}", self.kind, self.msg)
    }
}

macro_rules! impl_from {
    ($ty:ty, $id:ident) => {
        impl From<$ty> for RispError {
            fn from(e: $ty) -> Self {
                RispError {
                    kind: ErrorKind::$id,
                    msg: e.to_string().into_boxed_str(),
                }
            }
        }
    };
}

impl_from!(EvalError, EvalError);
impl_from!(ParseError, ParseError);
