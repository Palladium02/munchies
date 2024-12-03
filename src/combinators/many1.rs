use crate::{traits::Parser, types::ParseError};

#[derive(Debug, Clone, Copy)]
pub struct Many1<P> {
    parser: P,
}

impl<I, O, P> Parser<I, Vec<O>> for Many1<P>
where
    P: Parser<I, O> + Clone + Copy,
    I: Clone,
{
    fn parse(self, input: I) -> Result<(Vec<O>, I), ParseError> {
        let mut input = input;
        let mut output = Vec::new();
        while let Ok((value, rest)) = self.parser.parse(input.clone()) {
            output.push(value);
            input = rest;
        }

        if output.is_empty() {
            return Err(ParseError {
                message: "Expected at least one match".to_string(),
            });
        }

        Ok((output, input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::char;
    use crate::traits::Parser;

    #[test]
    fn test_many1() {
        let parser = Many1 {
            parser: char(|c| c == 'a'),
        };
        assert_eq!(parser.parse("a"), Ok((vec!['a'], "")));
        assert_eq!(parser.parse("aa"), Ok((vec!['a', 'a'], "")));
        assert_eq!(
            parser.parse("b"),
            Err(ParseError {
                message: "Expected at least one match".into()
            })
        );
    }
}
