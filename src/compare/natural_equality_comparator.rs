use super::equality_comparator::EqualityComparator;

pub struct NaturalEqualityComparator;

impl<Operand> EqualityComparator<Operand> for NaturalEqualityComparator
where
    Operand: Eq,
{
    fn are_equal(a: Operand, b: Operand) -> bool {
        a.eq(&b)
    }
}

#[cfg(test)]
mod tests {
    use super::super::equality_comparator::EqualityComparator;
    use super::NaturalEqualityComparator;

    #[test]
    fn test_int_eq() {
        assert!(NaturalEqualityComparator::are_equal(0, 0));
    }

    #[test]
    fn test_int_neq() {
        assert!(!NaturalEqualityComparator::are_equal(0, 1));
    }

    #[test]
    fn test_str_eq() {
        assert!(NaturalEqualityComparator::are_equal("a", "a"));
    }

    #[test]
    fn test_str_neq() {
        assert!(!NaturalEqualityComparator::are_equal("a", "b"));
    }
}
