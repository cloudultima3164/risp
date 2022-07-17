use std::collections::BTreeMap;

use super::parser::ast::Expression;

#[derive(Debug, Default)]
pub struct Environment {
    data: BTreeMap<Box<str>, Expression>,
}
