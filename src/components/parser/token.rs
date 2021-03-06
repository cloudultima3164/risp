use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub inner: TokenType,
    pub row: isize,
    pub column: isize,
}

impl Token {
    pub fn new(inner: TokenType, row: isize, column: isize) -> Self {
        Token { inner, row, column }
    }
}

pub enum TokenError<'a> {
    Unexpected(&'a Token),
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Ident(Box<str>),
    Symbol(Symbol),
    Unexpected(char),
}

macro_rules! symbol_enum_with_asref_repr {
	($($id:ident: $repr:expr),*) => {
		#[allow(unused_imports)]
		use strum::{self, AsRefStr, EnumIter, IntoEnumIterator};
		use std::convert::AsRef;

		#[derive(Debug, AsRefStr, EnumIter, PartialEq, Clone, Copy)]
		pub enum Symbol {
			$(
				#[strum(serialize = $repr)] $id,
			)*
		}
	}
}

symbol_enum_with_asref_repr! {
    OpenParen: "(",
    CloseParen: ")",
    Comma: ",",
    Add: "+",
    Subtract: "-",
    Multiply: "*",
    Divide: "/",
    ToPower: "^",
    ReduceOrEscape: r#"\"#,
    DoubleQuote: r#"""#,
    NewLine: "\n",
    CarriageReturn: "\r",
    WhiteSpace: " "
}

impl PartialEq<str> for Symbol {
    fn eq(&self, rhs: &str) -> bool {
        self.as_ref() == rhs
    }
}

impl PartialEq<char> for Symbol {
    fn eq(&self, rhs: &char) -> bool {
        self == rhs.to_string().as_str()
    }
}

impl TryFrom<&char> for Symbol {
    type Error = char;

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        Symbol::iter().find(|sym| sym == c).ok_or(*c)
    }
}

impl Symbol {
    pub fn symbols() -> Box<[Symbol]> {
        Symbol::iter()
            .fold(Vec::new(), |mut vec, next| {
                vec.push(next);
                vec
            })
            .into_boxed_slice()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_symbol_enum() {
        symbol_enum_with_asref_repr! {
            Wow: "Wow!"
        }
        assert_eq!("Wow!", Symbol::Wow.as_ref())
    }

    #[test]
    fn test_symbol_matching() {
        let to_match = ["(", ")", "_", "*"];
        let symbols = Symbol::symbols();

        let matched = to_match.map(|s| symbols.iter().find(|sym| *sym == s));

        assert_eq!(
            matched,
            [
                Some(&Symbol::OpenParen),
                Some(&Symbol::CloseParen),
                None,
                Some(&Symbol::Multiply)
            ]
        )
    }
}
