use super::ast::Expression::{self, *};

pub struct Func {
    pub name: &'static str,
    func: &'static dyn Function<Expression>,
}

pub const ADD: Func = Func::new("add", &add);
pub const SUBTRACT: Func = Func::new("subtract", &subtract);
pub const MULTIPLY: Func = Func::new("multiply", &multiply);
pub const DIVIDE: Func = Func::new("divide", &divide);

impl Func {
    pub const fn new(name: &'static str, func: &'static dyn Function<Expression>) -> Self {
        Func { name, func }
    }

    pub fn eval(&self, lhs: Expression, rhs: Expression) -> Result<Expression, EvalError> {
        (self.func).evaluate(&lhs, &rhs)
    }
}

pub enum EvalError {
    TypeMismatch,
    IdentOp,
}

pub trait Function<T> {
    fn evaluate(&'static self, lhs: &T, rhs: &T) -> Result<T, EvalError>;
}

// Serves as "boundary" for the eval loop.
impl<F: Fn(&Expression, &Expression) -> Result<Expression, EvalError>> Function<Expression> for F {
    fn evaluate(
        &'static self,
        lhs: &Expression,
        rhs: &Expression,
    ) -> Result<Expression, EvalError> {
        // Invariant: Only evaluate expressions of the same type.
        // If different types can be evaluated together, conversion should happen
        //  before this point is reached.
        if std::mem::discriminant(lhs) != std::mem::discriminant(rhs) {
            return Err(EvalError::TypeMismatch);
        }

        // Invariant: Idents can never be evaluated
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
