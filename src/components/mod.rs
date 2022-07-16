use self::{ast::Expression, env::Environment};

mod ast;
mod env;
mod error;
mod parser;

#[derive(Debug, Default)]
pub struct VirtualMachine<'a> {
    env: Environment,
    current_scope: &'a str,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(inherit: Option<Environment>) -> Self {
        inherit
            .map(|env| VirtualMachine {
                env,
                current_scope: "",
            })
            .unwrap_or_else(|| VirtualMachine::default())
    }

    pub fn run() {}

    // fn parse(&self) -> Expression {
    //     let tokens = self.tokenize();
    // }

    fn tokenize(&'a self) -> Box<[Box<str>]> {
        self.current_scope
            .replace('(', " ( ")
            .replace(')', " ) ")
            .split_whitespace()
            .map(|s| s.to_string().into_boxed_str())
            .collect()
    }
}
