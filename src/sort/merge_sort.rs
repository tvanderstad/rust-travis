use crate::util;

fn merge<T: Copy>(
    items: &mut [T],
    buffer: &mut [T],
    is_less: impl Fn(&T, &T) -> bool,
    s: usize,
    m: usize,
    e: usize,
) {
    let mut i = s;
    let mut j = m;
    let mut b = s;
    while i < m && j < e {
        if is_less(&items[i], &items[j]) {
            buffer[b] = items[i];
            i += 1;
        } else {
            buffer[b] = items[j];
            j += 1;
        }
        b += 1;
    }
    while i < m {
        buffer[b] = items[i];
        i += 1;
        b += 1;
    }
    while j < e {
        buffer[b] = items[j];
        j += 1;
        b += 1;
    }
    util::subslice_copy_aligned(buffer, items, s, e);
}

fn merge_sort_recursive<T: Copy>(
    items: &mut [T],
    buffer: &mut [T],
    is_less: &impl Fn(&T, &T) -> bool,
    s: usize,
    e: usize,
) {
    if e - s > 1 {
        let m = (e + s) / 2;
        merge_sort_recursive(items, buffer, is_less, s, m);
        merge_sort_recursive(items, buffer, is_less, m, e);
        merge(items, buffer, is_less, s, m, e);
    }
}

pub fn merge_sort<T: Copy>(items: &mut [T], is_less: &impl Fn(&T, &T) -> bool) {
    if items.len() > 1 {
        let mut buffer = vec![items[0]; items.len()];
        merge_sort_recursive(items, &mut buffer[..], is_less, 0, items.len());
    }
}

#[cfg(test)]
mod merge_sort_tests {
    use super::merge_sort;
    test! { merge_sort, &|x, y| x < y }
}