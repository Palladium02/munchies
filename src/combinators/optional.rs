use crate::{traits::Parser, types::ParseError};

#[derive(Debug, Clone, Copy)]
pub struct Optional<P> {
    parser: P,
}

impl<I, O, P> Parser<I, Option<O>> for Optional<P>
where
    P: Parser<I, O>,
    I: Clone,
{
    fn parse(self, input: I) -> Result<(Option<O>, I), ParseError> {
        match self.parser.parse(input.clone()) {
            Ok((output, rest)) => Ok((Some(output), rest)),
            Err(_) => Ok((None, input)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::char;
    use crate::traits::Parser;

    #[test]
    fn test_optional() {
        let parser = Optional {
            parser: char(|c| c == 'a'),
        };
        assert_eq!(parser.parse("a"), Ok((Some('a'), "")));
        assert_eq!(parser.parse("b"), Ok((None, "b")));
    }
}
