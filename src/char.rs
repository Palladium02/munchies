use crate::traits::Parser;

#[derive(Debug, Clone, Copy)]
pub struct Char<F> {
    predicate: F,
}

impl<'a, F> Parser<'a, char> for Char<F>
where
    F: Fn(char) -> bool,
{
    fn parse(&self, input: &'a str) -> Result<(char, &'a str), String> {
        let mut chars = input.chars();
        match chars.next() {
            Some(c) if (self.predicate)(c) => Ok((c, chars.as_str())),
            Some(c) => Err(format!("Unexpected character: {}", c)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}

pub fn char<F>(predicate: F) -> Char<F>
where
    F: Fn(char) -> bool,
{
    Char { predicate }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char() {
        let parser = char(|c| c == 'a');
        assert_eq!(parser.parse("abc"), Ok(('a', "bc")));
        assert_eq!(
            parser.parse("def"),
            Err("Unexpected character: d".to_string())
        );
        assert_eq!(parser.parse(""), Err("Unexpected end of input".to_string()));
    }
}
