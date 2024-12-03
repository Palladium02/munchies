use crate::{traits::Parser, types::ParseError};

#[derive(Debug, Clone, Copy)]
pub struct Byte<F> {
    f: F,
}

impl<F> Parser<&[u8], u8> for Byte<F>
where
    F: Fn(u8) -> bool,
{
    fn parse(self, input: &[u8]) -> Result<(u8, &[u8]), ParseError> {
        match input.first() {
            Some(&b) if (self.f)(b) => Ok((b, &input[1..])),
            Some(b) => Err(ParseError {
                message: format!("unexpected byte: {}", b),
            }),
            None => Err(ParseError {
                message: "unexpected end of input".to_string(),
            }),
        }
    }
}

pub fn byte<F>(f: F) -> Byte<F> {
    Byte { f }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte() {
        let a = byte(|b| b == b'a');
        assert_eq!(a.parse(b"abc"), Ok((b'a', &b"bc"[..])));
    }
}
