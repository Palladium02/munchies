use crate::traits::Parser;

pub fn run<'a, O>(parser: impl Parser<'a, O>, input: &'a str) -> Result<(O, &'a str), String>
where
    O: Clone,
{
    parser
        .parse(input)
        .iter()
        .min_by_key(|(_, rest)| rest.len())
        .map(|(output, rest)| Ok((output.clone(), *rest)))
        .unwrap_or(Err("No valid parse found.".to_string()))
}
