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

pub fn quick_sort<T: Copy>(items: &mut [T], is_less: &impl Fn(&T, &T) -> bool) {
    if items.len() > 1 {
        quick_sort_recursive(items, is_less, 0, items.len() - 1);
    }
}

#[cfg(test)]
mod quick_sort_tests {
    use super::quick_sort;
    test! { quick_sort, &|x, y| x < y }
}
