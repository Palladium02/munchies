use crate::{
    traits::Parser,
    types::{Either, ParseResult},
};

#[derive(Debug, Clone, Copy)]
pub struct Or<P1, P2> {
    left: P1,
    right: P2,
}

impl<'a, P1, P2, O1, O2> Parser<'a, Either<O1, O2>> for Or<P1, P2>
where
    P1: Parser<'a, O1>,
    P2: Parser<'a, O2>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Either<O1, O2>> {
        let left_result = self.left.parse(input);
        if !left_result.is_empty() {
            return left_result
                .into_iter()
                .map(|(o, rest)| (Either::Left(o), rest))
                .collect();
        }

        self.right
            .parse(input)
            .into_iter()
            .map(|(o, rest)| (Either::Right(o), rest))
            .collect()
    }
}

pub fn or<'a, P1, P2, O>(left: P1, right: P2) -> Or<P1, P2>
where
    P1: Parser<'a, O>,
    P2: Parser<'a, O>,
{
    Or { left, right }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::literal::literal;

    #[test]
    fn test_or() {
        let parser = or(literal("a"), literal("bc"));
        // assert_eq!(parser.parse("abc"), vec![("a", "bc")]);
        assert_eq!(parser.parse("abc"), vec![(Either::Left("a"), "bc")]);
    }
}
