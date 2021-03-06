use std::collections::BTreeMap;

use super::ast::Expression;

#[derive(Debug, Default)]
pub struct Environment {
    data: BTreeMap<Box<str>, Expression>,
}
