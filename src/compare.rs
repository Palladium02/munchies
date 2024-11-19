use crate::types::ParseResult;

pub(crate) fn compare_results<T>(left: ParseResult<T>, right: ParseResult<T>) -> bool
where
    T: PartialEq + Clone,
{
    if left.len() != right.len() {
        return false;
    }
    for (left, right) in left.iter().zip(right.iter()) {
        if left.0 != right.0 {
            return false;
        }
        if left.1 != right.1 {
            return false;
        }
    }
    return true;
}
