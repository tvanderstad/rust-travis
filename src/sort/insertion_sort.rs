use crate::util;

pub fn insertion_sort<T: Copy>(items: &mut [T], is_less: impl Fn(&T, &T) -> bool) {
    for i in 2..(items.len() + 1) {
        for j in (1..i).rev() {
            if is_less(&items[j], &items[j - 1]) {
                util::swap(items, j, j - 1);
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::insertion_sort;
    use crate::permute::steinhaus_johnson_trotter::SteinhausJohnsonTrotter;
    use crate::permute::Permutation;
    use crate::sort::is_sorted::is_sorted;
    use crate::sort::test::{TEST_LEN_COMBINATION, TEST_LEN_PERMUTATION};
    use itertools::Itertools;

    #[test]
    fn permutations() {
        for mut permutation in SteinhausJohnsonTrotter::new(TEST_LEN_PERMUTATION) {
            insertion_sort(&mut permutation.v, |x, y| x < y);
            assert_eq!(Permutation::new(TEST_LEN_PERMUTATION).v, permutation.v);
        }
    }

    #[test]
    fn combinations() {
        let mut multi_prod = (0..TEST_LEN_COMBINATION)
            .map(|_| 0..TEST_LEN_COMBINATION)
            .multi_cartesian_product();
        while let Some(mut combination) = multi_prod.next() {
            insertion_sort(&mut combination, |x, y| x < y);
            assert!(is_sorted(&combination, |x, y| x < y));
        }
    }
}
