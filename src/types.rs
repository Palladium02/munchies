pub type ParseResult<'a, O> = Vec<(O, &'a str)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
