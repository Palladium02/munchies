use crate::{helper::char, traits::Parser};

pub fn any<'a>() -> impl Parser<'a, char> {
    char(|_| true)
}
