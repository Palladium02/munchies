use crate::{traits::Parser, types::ParseError};

#[derive(Debug, Clone, Copy)]
pub struct Literal {
    literal: &'static str,
}

impl<'i, 'o> Parser<&'i str, &'o str> for Literal
where
    'i: 'o,
{
    fn parse(self, input: &'i str) -> Result<(&'o str, &'i str), ParseError> {
        if let Some(rest) = input.strip_prefix(self.literal) {
            Ok((&input[..self.literal.len()], rest))
        } else {
            Err(ParseError {
                message: format!("unexpected literal: {}", self.literal),
            })
        }
    }
}

pub fn literal(literal: &'static str) -> Literal {
    Literal { literal }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        let abc = literal("abc");
        assert_eq!(abc.parse("abcdef"), Ok(("abc", "def")));
    }
}
