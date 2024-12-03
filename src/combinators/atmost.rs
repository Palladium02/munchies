use crate::{traits::Parser, types::ParseError};

#[derive(Debug, Clone, Copy)]
pub struct AtMost<P> {
    parser: P,
    n: usize,
}

impl<I, O, P> Parser<I, Vec<O>> for AtMost<P>
where
    P: Parser<I, O> + Clone + Copy,
    I: Clone,
{
    fn parse(self, input: I) -> Result<(Vec<O>, I), ParseError> {
        let mut remainder = input;
        let mut outputs = Vec::new();

        for _ in 0..self.n {
            match self.parser.parse(remainder.clone()) {
                Ok((output, rest)) => {
                    outputs.push(output);
                    remainder = rest;
                }
                Err(_) => break,
            }
        }

        Ok((outputs, remainder))
    }
}

pub fn at_most<P>(n: usize, parser: P) -> AtMost<P> {
    AtMost { parser, n }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::char;
    use crate::traits::Parser;

    #[test]
    fn test_at_most() {
        let parser = at_most(2, char(|c| c == 'a'));
        assert_eq!(parser.parse("a"), Ok((vec!['a'], "")));
        assert_eq!(parser.parse("aa"), Ok((vec!['a', 'a'], "")));
        assert_eq!(parser.parse("aaa"), Ok((vec!['a', 'a'], "a")));
        assert_eq!(parser.parse("aaaa"), Ok((vec!['a', 'a'], "aa")));
        assert_eq!(parser.parse("b"), Ok((vec![], "b")));
    }
}
