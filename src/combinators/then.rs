use crate::{traits::Parser, types::ParseError};

#[derive(Debug, Clone, Copy)]
pub struct Then<First, Second> {
    first: First,
    second: Second,
}

impl<I, O1, O2, First, Second> Parser<I, (O1, O2)> for Then<First, Second>
where
    First: Parser<I, O1>,
    Second: Parser<I, O2>,
    I: Clone,
{
    fn parse(self, input: I) -> Result<((O1, O2), I), ParseError> {
        let (output1, rest1) = self.first.parse(input.clone())?;
        let (output2, rest2) = self.second.parse(rest1)?;
        Ok(((output1, output2), rest2))
    }
}

pub fn then<First, Second>(first: First, second: Second) -> Then<First, Second> {
    Then { first, second }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::char;
    use crate::traits::Parser;
    use crate::types::ParseError;

    #[test]
    fn test_then() {
        let parser = then(char(|c| c == 'a'), char(|c| c == 'b'));
        assert_eq!(parser.parse("ab"), Ok((('a', 'b'), "")));
        assert_eq!(
            parser.parse("ac"),
            Err(ParseError {
                message: "unexpected character: c".into()
            })
        );
        assert_eq!(
            parser.parse("bc"),
            Err(ParseError {
                message: "unexpected character: b".into()
            })
        );
    }
}
