use either::Either;

use crate::{traits::Parser, types::ParseError};

#[derive(Debug, Clone, Copy)]
pub struct Or<P1, P2> {
    left: P1,
    right: P2,
}

impl<I, O1, O2, P1, P2> Parser<I, Either<O1, O2>> for Or<P1, P2>
where
    P1: Parser<I, O1>,
    P2: Parser<I, O2>,
    I: Clone,
{
    fn parse(self, input: I) -> Result<(Either<O1, O2>, I), ParseError> {
        match self.left.parse(input.clone()) {
            Ok((output, rest)) => Ok((Either::Left(output), rest)),
            Err(_) => match self.right.parse(input) {
                Ok((output, rest)) => Ok((Either::Right(output), rest)),
                Err(_) => Err(ParseError {
                    message: "Expected one of the options to match".to_string(),
                }),
            },
        }
    }
}

pub fn or<P1, P2>(left: P1, right: P2) -> Or<P1, P2> {
    Or { left, right }
}
