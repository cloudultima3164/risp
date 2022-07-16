pub enum ErrorKind {
    Other,
    EvalError,
    ParseError,
}

pub struct Error {
    kind: ErrorKind,
    msg: Box<str>,
}
