use crate::traits::Parser;

#[derive(Debug, Clone, Copy)]
pub struct Literal<'a> {
    literal: &'a str,
}

impl<'a> Parser<'a, &'a str> for Literal<'a> {
    fn parse(&self, input: &'a str) -> Result<(&'a str, &'a str), String> {
        if input.starts_with(self.literal) {
            Ok((self.literal, &input[self.literal.len()..]))
        } else {
            Err(format!("Expected '{}'", self.literal))
        }
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
        assert_eq!(parser.parse("abcdef"), Ok(("abc", "def")));
        assert_eq!(parser.parse("def"), Err("Expected 'abc'".to_string()));
        assert_eq!(parser.parse(""), Err("Expected 'abc'".to_string()));
    }
}
