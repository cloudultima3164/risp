use self::{env::Environment, error::RispError};

mod env;
mod error;
mod interpreter;

#[derive(Debug, Default)]
pub struct VirtualMachine {
    env: Environment,
    current_scope: Box<str>,
}

impl VirtualMachine {
    pub fn new(inherit: Option<Environment>) -> Self {
        inherit
            .map(|env| VirtualMachine {
                env,
                current_scope: Box::default(),
            })
            .unwrap_or_else(VirtualMachine::default)
    }

    pub fn run(&mut self, scope: &str) -> Result<(), RispError> {
        self.current_scope = scope.to_string().into_boxed_str();

        // Parsing
        let mut parser = interpreter::parser::Parser::new();
        parser.parse(&self.current_scope)?;

        // Building AST
        let mut ast = interpreter::ast::Ast::new();
        ast.build(parser)?;

        Ok(())
    }
}
