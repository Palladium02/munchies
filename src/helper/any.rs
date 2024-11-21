use crate::{helper::char, traits::Parser};

pub fn any<'a>() -> impl Parser<'a, char> {
    char(|_| true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compare::compare_results;

    #[test]
    fn test_any() {
        let parser = any();
        let result = parser.parse("abc");
        assert!(compare_results(result, vec![('a', "bc")]));
    }
}
