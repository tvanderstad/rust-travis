use crate::util;

pub fn partition<T: Copy>(
    items: &mut [T],
    is_less: &impl Fn(&T, &T) -> bool,
    s: usize,
    e: usize,
) -> usize {
    let pivot = items[(e + s) / 2];
    let mut i = s;
    let mut j = e;
    loop {
        while is_less(&items[i], &pivot) && i < j {
            i += 1;
        }
        while is_less(&pivot, &items[j]) && i < j {
            j -= 1;
        }
        util::swap(items, i, j);
        if j - i < 2 {
            break;
        }
        i += 1;
        j -= 1;
    }
    return i;
}

pub fn quick_sort_recursive<T: Copy>(
    items: &mut [T],
    is_less: &impl Fn(&T, &T) -> bool,
    s: usize,
    e: usize,
) {
    let m = partition(items, is_less, s, e);
    if s + 1 < m && m > 1 {
        quick_sort_recursive(items, is_less, s, m - 1);
    }
    if m + 1 < e {
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