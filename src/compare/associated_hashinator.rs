use super::hashinator::Hashinator;
use std::hash::{Hash, Hasher};

pub struct AssociatedHashinator;

impl<Operand, OperandHasher: Hasher> Hashinator<Operand, OperandHasher> for AssociatedHashinator
where
    Operand: Hash,
{
    fn hash(a: &Operand, hasher: &mut OperandHasher) {
        a.hash(hasher);
    }
}

#[cfg(test)]
mod tests {
    use super::super::hashinator::Hashinator;
    use super::AssociatedHashinator;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    macro_rules! test_case {
        ($test_name:ident: $value:expr) => {
            #[test]
            fn $test_name() {
                let (value1, value2, expected_result) = $value;

                let mut hasher = DefaultHasher::new();
                AssociatedHashinator::hash(&value1, &mut hasher);
                let hash1 = hasher.finish();

                let mut hasher = DefaultHasher::new();
                AssociatedHashinator::hash(&value2, &mut hasher);
                let hash2 = hasher.finish();

                if expected_result {
                    assert_eq!(hash1, hash2);
                } else {
                    assert_ne!(hash1, hash2);
                }
            }
        };
    }

    test_case! {int_eq: (0i32, 0i32, true)}
    test_case! {int_ne: (0i32, 1i32, false)}
    test_case! {str_eq: ("a", "a", true)}
    test_case! {str_ne: ("a", "b", false)}
}
