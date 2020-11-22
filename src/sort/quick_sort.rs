use crate::util;

pub fn partition<T: Copy>(
    items: &mut [T],
    is_less: &impl Fn(&T, &T) -> bool,
    s: usize,
    e: usize,
) -> usize {
    let pivot = items[(s + e) / 2];
    let mut i = s;
    let mut j = e;
    loop {
        while is_less(&items[i], &pivot) {
            i += 1;
        }
        while is_less(&pivot, &items[j]) {
            j -= 1;
        }
        if i >= j {
            return j;
        }
        util::swap(items, i, j);
        i += 1;
        j -= 1;
    }
}

pub fn quick_sort_recursive<T: Copy>(
    items: &mut [T],
    is_less: &impl Fn(&T, &T) -> bool,
    s: usize,
    e: usize,
) {
    if e > s {
        let m = partition(items, is_less, s, e);
        quick_sort_recursive(items, is_less, s, m);
        quick_sort_recursive(items, is_less, m + 1, e);
    }
}

pub fn quick_sort<T: Copy>(items: &mut [T], is_less: impl Fn(&T, &T) -> bool) {
    if items.len() > 1 {
        quick_sort_recursive(items, &is_less, 0, items.len() - 1);
    }
}

#[cfg(test)]
mod test {
    use super::quick_sort;
    use crate::permute::steinhaus_johnson_trotter::SteinhausJohnsonTrotter;
    use crate::permute::Permutation;
    use crate::sort::is_sorted::is_sorted;
    use crate::sort::test::{TEST_LEN_COMBINATION, TEST_LEN_PERMUTATION};
    use itertools::Itertools;

    #[test]
    fn permutations() {
        for mut permutation in SteinhausJohnsonTrotter::new(TEST_LEN_PERMUTATION) {
            quick_sort(&mut permutation.v, |x, y| x < y);
            assert_eq!(Permutation::new(TEST_LEN_PERMUTATION).v, permutation.v);
        }
    }

    #[test]
    fn combinations() {
        let mut multi_prod = (0..TEST_LEN_COMBINATION)
            .map(|_| 0..TEST_LEN_COMBINATION)
            .multi_cartesian_product();
        while let Some(mut combination) = multi_prod.next() {
            quick_sort(&mut combination, |x, y| x < y);
            assert!(is_sorted(&combination, |x, y| x < y));
        }
    }
}
