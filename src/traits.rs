use crate::types::ParseError;

pub trait Parser<I, O> {
    fn parse(self, input: I) -> Result<(O, I), ParseError>;
}
