use super::Expression::{self, *};

pub type ConstOp = (&'static str, &'static dyn Function<Expression>);

pub const ADD: ConstOp = ("add", &add);
pub const SUBTRACT: ConstOp = ("subtract", &subtract);
pub const MULTIPLY: ConstOp = ("multiply", &multiply);
pub const DIVIDE: ConstOp = ("divide", &divide);

pub enum EvalError {
    TypeMismatch,
    IdentOp,
}

pub trait Function<T> {
    fn evaluate(&'static self, lhs: &T, rhs: &T) -> Result<T, EvalError>
    where
        Self: Fn(&T, &T) -> Result<T, EvalError>,
    {
        (self)(lhs, rhs)
    }
}

impl<Func> Function<Expression> for Func {
    fn evaluate(&'static self, lhs: &Expression, rhs: &Expression) -> Result<Expression, EvalError>
    where
        Func: Fn(&Expression, &Expression) -> Result<Expression, EvalError>,
    {
        if std::mem::discriminant(lhs) != std::mem::discriminant(rhs) {
            return Err(EvalError::TypeMismatch);
        }
        if let Expression::Ident(_) = lhs {
            return Err(EvalError::IdentOp);
        }
        (self)(lhs, rhs)
    }
}

macro_rules! simple_math {
	($name:ident, $operator:tt) => {
		pub fn $name(lhs: &Expression, rhs: &Expression) -> Result<Expression, EvalError> {
			let expr = match (lhs, rhs) {
				(Number(l), Number(r)) => Number(l $operator r),
				(List(l), List(r)) => {
					let mut new_list = Vec::new();
					for (l, r) in l.iter().zip(r) {
						new_list.push($name.evaluate(l, r)?)
					}
					List(new_list)
				},
				_ => unreachable!()
			};
			Ok(expr)
		}
	}
}

simple_math!(add, +);
simple_math!(subtract, -);
simple_math!(multiply, *);
simple_math!(divide, /);
