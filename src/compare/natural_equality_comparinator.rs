use super::equality_comparinator::EqualityComparinator;

pub struct NaturalEqualityComparinator;

impl<Operand> EqualityComparinator<Operand> for NaturalEqualityComparinator
where
    Operand: Eq,
{
    fn are_equal(a: Operand, b: Operand) -> bool {
        a.eq(&b)
    }
}

#[cfg(test)]
mod tests {
    use super::super::equality_comparinator::EqualityComparinator;
    use super::NaturalEqualityComparinator;

    #[test]
    fn test_int_eq() {
        assert!(NaturalEqualityComparinator::are_equal(0, 0));
    }

    #[test]
    fn test_int_neq() {
        assert!(!NaturalEqualityComparinator::are_equal(0, 1));
    }

    #[test]
    fn test_str_eq() {
        assert!(NaturalEqualityComparinator::are_equal("a", "a"));
    }

    #[test]
    fn test_str_neq() {
        assert!(!NaturalEqualityComparinator::are_equal("a", "b"));
    }
}
