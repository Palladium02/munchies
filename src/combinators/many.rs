use crate::{traits::Parser, types::ParseResult};

#[derive(Debug, Clone, Copy)]
pub struct Many<P> {
    parser: P,
}

impl<'a, P, O> Parser<'a, Vec<O>> for Many<P>
where
    O: Clone,
    P: Parser<'a, O>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Vec<O>> {
        let mut results: Vec<(Vec<O>, &'a str)> = Vec::new();
        let mut remainder = input;

        loop {
            let local_results = self.parser.parse(remainder);
            if local_results.is_empty() {
                break;
            }

            let (match_result, rest) = local_results.get(0).expect("");
            let (mut most_recent_results, _) =
                results.last().unwrap_or(&(vec![], remainder)).clone();

            most_recent_results.push(match_result.clone());
            results.push((most_recent_results, rest));
            remainder = rest;
        }

        results
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
    use crate::combinators::then::then;
    use crate::compare::compare_results;
    use crate::helper::char::char;

    #[test]
    fn test_many() {
        let parser = many(char(|c| c == 'a'));
        assert_eq!(parser.parse("aaabc"), vec![(vec!['a', 'a', 'a'], "bc")]);
    }

    #[test]
    fn test_many_then_ambiguous() {
        let a = char(|c| c == 'a');
        let many_as = many(a);
        let ambiguous_p = then(many_as, a);

        let result = ambiguous_p.parse("aaaabc");
        assert!(compare_results(
            result,
            vec![
                ((vec!['a', 'a', 'a'], 'a'), "bc"),
                ((vec!['a', 'a'], 'a'), "abc"),
                ((vec!['a'], 'a'), "aabc"),
            ]
        ))
    }
}
