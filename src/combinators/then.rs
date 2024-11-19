use crate::traits::Parser;

#[derive(Debug, Clone, Copy)]
pub struct Then<P1, P2> {
    left: P1,
    right: P2,
}

impl<'a, P1, P2, O1, O2> Parser<'a, (O1, O2)> for Then<P1, P2>
where
    P1: Parser<'a, O1>,
    P2: Parser<'a, O2>,
{
    fn parse(&self, input: &'a str) -> Result<((O1, O2), &'a str), String> {
        let (left_output, remainder) = self.left.parse(input)?;
        let (right_output, remainder) = self.right.parse(remainder)?;
        Ok(((left_output, right_output), remainder))
    }
}

pub fn then<'a, P1, P2, O1, O2>(left: P1, right: P2) -> Then<P1, P2>
where
    P1: Parser<'a, O1>,
    P2: Parser<'a, O2>,
{
    Then { left, right }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char::char;
    use crate::literal::literal;

    #[test]
    fn test_then() {
        let parser = then(char(|c| c == 'a'), literal("bc"));
        assert_eq!(parser.parse("abcd"), Ok((('a', "bc"), "d")));
        assert_eq!(parser.parse("ab"), Err("Expected 'bc'".to_string()));
        assert_eq!(parser.parse("a"), Err("Expected 'bc'".to_string()));
        assert_eq!(
            parser.parse("b"),
            Err("Unexpected character: b".to_string())
        );
    }
}
