use crate::traits::Parser;

#[derive(Debug, Clone, Copy)]
pub struct Many<P> {
    parser: P,
}

impl<'a, P, O> Parser<'a, Vec<O>> for Many<P>
where
    P: Parser<'a, O>,
{
    fn parse(&self, input: &'a str) -> Result<(Vec<O>, &'a str), String> {
        let mut results = Vec::new();
        let mut remainder = input;

        while let Ok((output, next_remainder)) = self.parser.parse(&remainder) {
            if next_remainder == remainder {
                break;
            }

            remainder = next_remainder;
            results.push(output);
        }

        Ok((results, remainder))
    }
}

pub fn many<'a, P, O>(parser: P) -> Many<P>
where
    P: Parser<'a, O>,
{
    Many { parser }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::char::char;
    use crate::combinators::then::then;

    #[test]
    fn test_many() {
        let parser = many(char(|c| c == 'a'));
        assert_eq!(parser.parse("aaaabc"), Ok((vec!['a', 'a', 'a'], "bc")));
        assert_eq!(parser.parse("bc"), Ok((vec![], "bc")));
        assert_eq!(parser.parse("def"), Ok((vec![], "def")));
        assert_eq!(parser.parse(""), Ok((vec![], "")));
    }

    #[test]
    fn test_many_then_ambiguous() {
        let a = char(|c| c == 'a');
        let many_as = many(a);
        let ambiguous_p = then(many_as, a);
        assert_eq!(ambiguous_p.parse("aaab"), Ok(((vec!['a', 'a'], 'a'), "b")))
    }
}
