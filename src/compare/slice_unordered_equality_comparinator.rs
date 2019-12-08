use super::equality_comparinator::EqualityComparinator;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct SliceUnorderedEqualityComparinator;

// TODO: parametrize with an equality comparinator
impl<Operand: Eq + Hash + Copy> EqualityComparinator<&[Operand]>
    for SliceUnorderedEqualityComparinator
{
    fn are_equal(a: &[Operand], b: &[Operand]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut a_histogram: HashMap<Operand, u64> = HashMap::new();
        let mut b_histogram: HashMap<Operand, u64> = HashMap::new();

        for o in a.iter() {
            let current = match a_histogram.get(o) {
                Some(n) => *n,
                None => 0u64,
            };
            a_histogram.remove(o);
            a_histogram.insert(*o, current + 1);
        }
        for o in b.iter() {
            let current = match b_histogram.get(o) {
                Some(n) => *n,
                None => 0u64,
            };
            b_histogram.remove(o);
            b_histogram.insert(*o, current + 1);
        }

        let a_keys: HashSet<Operand> = a_histogram.keys().copied().collect();
        let b_keys: HashSet<Operand> = b_histogram.keys().copied().collect();
        for o in a_keys.union(&b_keys) {
            match (a_histogram.get(o), b_histogram.get(o)) {
                (Some(n), Some(m)) => {
                    if m != n {
                        return false;
                    }
                }
                (None, Some(_)) => {
                    return false;
                }
                (Some(_), None) => {
                    return false;
                }
                (None, None) => (),
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::super::equality_comparinator::EqualityComparinator;
    use super::SliceUnorderedEqualityComparinator;

    macro_rules! test_case {
        ($test_name:ident: $value:expr) => {
            #[test]
            fn $test_name() {
                let (input1, input2, expected_result) = $value;
                assert_eq!(
                    expected_result,
                    SliceUnorderedEqualityComparinator::are_equal(&input1, &input2)
                );
            }
        };
    }

    test_case! { both_empty: ([], [0i32; 0], true)}
    test_case! { one_empty: ([], [99], false)}
    test_case! { single_same: ([99], [99], true)}
    test_case! { single_different: ([99], [100], false)}
    test_case! { double_same: ([99, 100], [99, 100], true)}
    test_case! { double_different: ([99, 100], [99, 101], false)}
    test_case! { different_length: ([99], [99, 100], false)}
    test_case! { different_length_duplicate: ([99], [99, 99], false)}
    test_case! { one_duplicate: ([99, 99], [99, 100], false)}
    test_case! { both_same_duplicate: ([99, 99], [99, 99], true)}
    test_case! { both_different_duplicate: ([99, 99], [100, 100], false)}
    test_case! { both_same_duplicate_extra_different: ([99, 99, 101], [99, 99, 100], false)}
    test_case! { both_same_duplicate_extra_same: ([99, 99, 100], [99, 99, 100], true)}
    test_case! { both_different_duplicate_extra_same: ([99, 99, 101], [100, 100, 101], false)}
    test_case! { double_same_reordered: ([99, 100], [100, 99], true)}
    test_case! { both_same_duplicate_extra_same_reordered: ([99, 99, 100], [100, 99, 99], true)}
}
