use crate::{traits::Parser, types::ParseError};

#[derive(Debug, Clone, Copy)]
pub struct Char<F> {
    f: F,
}

impl<F> Parser<&str, char> for Char<F>
where
    F: Fn(char) -> bool,
{
    fn parse(self, input: &str) -> Result<(char, &str), ParseError> {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) if (self.f)(c) => Ok((c, chars.as_str())),
            Some(c) => Err(ParseError {
                message: format!("unexpected character: {}", c),
            }),
            None => Err(ParseError {
                message: "unexpected end of input".to_string(),
            }),
        }
    }
}

pub fn char<F>(f: F) -> Char<F> {
    Char { f }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char() {
        let a = char(|c| c == 'a');
        assert_eq!(a.parse("abc"), Ok(('a', "bc")));
    }
}
