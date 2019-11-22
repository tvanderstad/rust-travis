pub trait EqualityComparator<T> {
    fn are_equal(a: &T, b: &T) -> bool;
}

pub struct NaturalEqualityComparator {}

impl<T> EqualityComparator<T> for NaturalEqualityComparator where T: Eq {
    fn are_equal(a: &T, b: &T) -> bool {
        a.eq(b)
    }
}