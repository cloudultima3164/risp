use self::env::Environment;

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
}
