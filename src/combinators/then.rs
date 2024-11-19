use crate::{traits::Parser, types::ParseResult};

#[derive(Debug, Clone, Copy)]
pub struct Then<P1, P2> {
    left: P1,
    right: P2,
}

impl<'a, P1, P2, O1, O2> Parser<'a, (O1, O2)> for Then<P1, P2>
where
    O1: Clone,
    O2: Clone,
    P1: Parser<'a, O1>,
    P2: Parser<'a, O2>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, (O1, O2)> {
        let mut results = vec![];
        for (left_result, rest) in self.left.parse(input) {
            for (right_result, rest) in self.right.parse(rest) {
                results.push(((left_result.clone(), right_result), rest));
            }
        }

        results
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
    use crate::helper::char::char;
    use crate::helper::literal::literal;

    #[test]
    fn test_then() {
        let parser = then(char(|c| c == 'a'), literal("bc"));
        assert_eq!(parser.parse("abcdef"), vec![(('a', "bc"), "def")]);
    }
}
