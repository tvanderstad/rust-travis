pub trait EqualityComparator<T> {
    fn are_equal(a: &T, b: &T) -> bool;
}