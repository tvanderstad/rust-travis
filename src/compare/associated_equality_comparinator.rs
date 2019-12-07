use super::equality_comparinator::EqualityComparinator;

pub struct AssociatedEqualityComparinator;

impl<Operand> EqualityComparinator<Operand> for AssociatedEqualityComparinator
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
    use super::AssociatedEqualityComparinator;

    #[test]
    fn test_int_eq() {
        assert!(AssociatedEqualityComparinator::are_equal(0, 0));
    }

    #[test]
    fn test_int_neq() {
        assert!(!AssociatedEqualityComparinator::are_equal(0, 1));
    }

    #[test]
    fn test_str_eq() {
        assert!(AssociatedEqualityComparinator::are_equal("a", "a"));
    }

    #[test]
    fn test_str_neq() {
        assert!(!AssociatedEqualityComparinator::are_equal("a", "b"));
    }
}
