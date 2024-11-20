use crate::{traits::Parser, types::ParseResult};

#[derive(Debug, Clone, Copy)]
pub struct Exactly<P> {
    parser: P,
    count: usize,
}

impl<'a, P, O> Parser<'a, Vec<O>> for Exactly<P>
where
    O: Clone,
    P: Parser<'a, O>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Vec<O>> {
        let mut results: Vec<(Vec<O>, &'a str)> = Vec::new();
        let mut remainder = input;

        for _ in 0..self.count {
            let local_results = self.parser.parse(remainder);
            if local_results.is_empty() {
                return Vec::new();
            }

            let (match_result, rest) = local_results.get(0).expect("");
            let (mut most_recent_results, _) = results.pop().unwrap_or((vec![], remainder)).clone();

            most_recent_results.push(match_result.clone());
            results.push((most_recent_results, rest));
            remainder = rest;
        }

        results
    }
}

pub fn exactly<'a, P, O>(parser: P, n: usize) -> Exactly<P>
where
    P: Parser<'a, O>,
{
    Exactly { parser, count: n }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::char::char;

    #[test]
    fn test_exactly() {
        let parser = exactly(char(|c| c == 'a'), 3);
        assert_eq!(parser.parse("aaabc"), vec![(vec!['a', 'a', 'a'], "bc")]);
    }
}
