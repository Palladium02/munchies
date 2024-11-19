use crate::{traits::Parser, types::ParseResult};

#[derive(Debug, Clone, Copy)]
pub struct Char<F> {
    predicate: F,
}

impl<'a, F> Parser<'a, char> for Char<F>
where
    F: Fn(char) -> bool,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, char> {
        if let Some(first) = input.chars().next() {
            if (self.predicate)(first) {
                return vec![(first, &input[first.len_utf8()..])];
            }
        }

        vec![]
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
        assert_eq!(parser.parse("abc"), vec![('a', "bc")]);
    }
}
