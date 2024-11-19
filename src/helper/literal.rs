use crate::{traits::Parser, types::ParseResult};

#[derive(Debug, Clone, Copy)]
pub struct Literal<'a> {
    literal: &'a str,
}

impl<'a> Parser<'a, &'a str> for Literal<'a> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, &'a str> {
        if input.starts_with(self.literal) {
            return vec![(self.literal, &input[self.literal.len()..])];
        }

        vec![]
    }
}

pub fn literal(literal: &str) -> Literal {
    Literal { literal }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        let parser = literal("abc");
        assert_eq!(parser.parse("abcdef"), vec![("abc", "def")]);
    }
}
