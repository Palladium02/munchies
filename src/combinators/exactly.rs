use crate::{traits::Parser, types::ParseError};

#[derive(Debug, Clone, Copy)]
pub struct Exactly<P> {
    parser: P,
    n: usize,
}

impl<I, O, P> Parser<I, Vec<O>> for Exactly<P>
where
    P: Parser<I, O> + Clone + Copy,
    I: Clone,
{
    fn parse(self, input: I) -> Result<(Vec<O>, I), ParseError> {
        let mut remainder = input;
        let mut outputs = Vec::new();
        for _ in 0..self.n {
            let (output, rest) = self.parser.parse(remainder.clone())?;
            outputs.push(output);
            remainder = rest;
        }

        Ok((outputs, remainder))
    }
}

pub fn exactly<P>(n: usize, parser: P) -> Exactly<P> {
    Exactly { parser, n }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::char;
    use crate::traits::Parser;
    use crate::types::ParseError;

    #[test]
    fn test_exactly() {
        let parser = exactly(2, char(|c| c == 'a'));
        assert_eq!(parser.parse("aa"), Ok((vec!['a', 'a'], "")));
        assert_eq!(parser.parse("aaa"), Ok((vec!['a', 'a'], "a")));
        assert_eq!(
            parser.parse("a"),
            Err(ParseError {
                message: "unexpected end of input".into()
            })
        );
    }
}
