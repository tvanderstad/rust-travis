use super::equality_comparer::EqualityComparator;

pub struct NaturalEqualityComparator {}

impl<T> EqualityComparator<T> for NaturalEqualityComparator where T: Eq {
    fn are_equal(a: &T, b: &T) -> bool {
        a.eq(b)
    }
}

#[cfg(test)]
mod tests {
    use super::super::equality_comparer::EqualityComparator;
    use super::NaturalEqualityComparator;

    #[test]
    fn test_int_eq() {
        assert!(NaturalEqualityComparator::are_equal(&0, &0));
    }

    #[test]
    fn test_int_neq() {
        assert!(!NaturalEqualityComparator::are_equal(&0, &1));
    }

    #[test]
    fn test_str_eq() {
        assert!(NaturalEqualityComparator::are_equal(&"a", &"a"));
    }

    #[test]
    fn test_str_neq() {
        assert!(!NaturalEqualityComparator::are_equal(&"a", &"b"));
    }
}