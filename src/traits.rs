pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> Result<(Output, &'a str), String>;
}
