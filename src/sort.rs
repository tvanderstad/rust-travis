use std::marker::Copy;

fn swap<T: Copy>(items: &mut [T], i: usize, j: usize) {
    let tmp = items[i];
    items[i] = items[j];
    items[j] = tmp;
}

pub fn insertion_sort<T: Copy>(items: &mut [T], is_less: &impl Fn(&T, &T) -> bool) {
    for i in 2..(items.len() + 1) {
        for j in (1..i).rev() {
            if is_less(&items[j], &items[j - 1]) {
                swap(items, j, j - 1);
            } else {
                break;
            }
        }
    }
}

pub fn selection_sort<T: Copy>(items: &mut [T], is_less: &impl Fn(&T, &T) -> bool) {
    for i in 1..items.len() {
        let mut min = i - 1;
        for j in i..items.len() {
            if is_less(&items[j], &items[min]) {
                min = j;
            }
        }
        if is_less(&items[min], &items[i - 1]) {
            swap(items, i - 1, min);
        }
    }
}

pub fn bubble_sort<T: Copy>(items: &mut [T], is_less: &impl Fn(&T, &T) -> bool) {
    for _ in 1..items.len() {
        for i in 1..items.len() {
            if is_less(&items[i], &items[i - 1]) {
                swap(items, i - 1, i);
            }
        }
    }
}

fn subslice_copy_aligned<T: Copy>(src: &[T], dest: &mut [T], s: usize, e: usize) {
    for i in s..e {
        dest[i] = src[i];
    }
}

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
    subslice_copy_aligned(buffer, items, s, e);
}

fn merge_sort_with_buffer<T: Copy>(
    items: &mut [T],
    buffer: &mut [T],
    is_less: &impl Fn(&T, &T) -> bool,
    s: usize,
    e: usize,
) {
    if e - s > 1 {
        let m = (e + s) / 2;
        merge_sort_with_buffer(items, buffer, is_less, s, m);
        merge_sort_with_buffer(items, buffer, is_less, m, e);
        merge(items, buffer, is_less, s, m, e);
    }
}

pub fn merge_sort<T: Copy>(items: &mut [T], is_less: &impl Fn(&T, &T) -> bool) {
    if items.len() > 1 {
        let mut buffer = vec![items[0]; items.len()];
        merge_sort_with_buffer(items, &mut buffer[..], is_less, 0, items.len());
    }
}

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
        swap(items, i, j);
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

#[macro_use]
#[cfg(test)]
mod test_macros {
    macro_rules! int_test_cases {
        ($algo:expr, $is_less:expr) => {
            algo_test_cases! {
                $algo,
                $is_less,
                empty: (&mut [], &[0i32; 0]),
                single: (&mut [0], &[0]),
                sorted: (&mut [0, 1, 2], &[0, 1, 2]),
                reverse_sorted: (&mut [2, 1, 0], &[0, 1, 2]),
                all_duplicate: (&mut [0, 0, 0], &[0, 0, 0]),
                some_duplicate: (&mut [0, 1, 0], &[0, 0, 1]),
                some_duplicate_sorted: (&mut [0, 0, 1], &[0, 0, 1]),
                some_duplicate_reverse_sorted: (&mut [1, 0, 0], &[0, 0, 1]),
                alternating_duplicate: (&mut [0, 1, 0, 1, 0], &[0, 0, 0, 1, 1]),
            }
        };
    }

    macro_rules! algo_test_cases {
        ($algo:expr, $is_less:expr, $($test_name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $test_name() {
                let (input, expected) = $value;
                $algo(input, $is_less);
                assert_eq!(expected, input);
            }
        )*
        }
    }
}

#[cfg(test)]
mod selection_sort_tests {
    use super::selection_sort;
    int_test_cases! { selection_sort, &|x, y| x < y }
}

#[cfg(test)]
mod insertion_sort_tests {
    use super::insertion_sort;
    int_test_cases! { insertion_sort, &|x, y| x < y }
}

#[cfg(test)]
mod bubble_sort_tests {
    use super::bubble_sort;
    int_test_cases! { bubble_sort, &|x, y| x < y }
}

#[cfg(test)]
mod merge_sort_tests {
    use super::merge_sort;
    int_test_cases! { merge_sort, &|x, y| x < y }
}

#[cfg(test)]
mod quick_sort_tests {
    use super::quick_sort;
    int_test_cases! { quick_sort, &|x, y| x < y }
}
