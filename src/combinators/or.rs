use crate::traits::Parser;

#[derive(Debug, Clone, Copy)]
pub struct Or<P1, P2> {
    left: P1,
    right: P2,
}

impl<'a, P1, P2, O> Parser<'a, O> for Or<P1, P2>
where
    P1: Parser<'a, O>,
    P2: Parser<'a, O>,
{
    fn parse(&self, input: &'a str) -> Result<(O, &'a str), String> {
        self.left.parse(input).or_else(|_| self.right.parse(input))
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
    use crate::literal::literal;

    #[test]
    fn test_or() {
        let parser = or(literal("a"), literal("bc"));
        assert_eq!(parser.parse("abc"), Ok(("a", "bc")));
        assert_eq!(parser.parse("bc"), Ok(("bc", "")));
        assert_eq!(
            parser.parse("def"),
            Err("Unexpected character: d".to_string())
        );
        assert_eq!(parser.parse(""), Err("Unexpected end of input".to_string()));
    }
}
