use std::marker::Copy;

fn swap<T : Copy>(items: &mut [T], i: usize, j: usize) {
    let tmp = items[i];
    items[i] = items[j];
    items[j] = tmp;
}

pub fn insertion_sort<T : Copy>(items: &mut [T], is_less: impl Fn(&T, &T) -> bool) {
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

pub fn selection_sort<T : Copy>(items: &mut [T], is_less: impl Fn(&T, &T) -> bool) {
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

pub fn bubble_sort<T : Copy>(items: &mut [T], is_less: impl Fn(&T, &T) -> bool) {
    for _ in 1..items.len() {
        for i in 1..items.len() {
            if is_less(&items[i], &items[i - 1]) {
                swap(items, i - 1, i);
            }
        }
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
            }
        }
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
    int_test_cases! { selection_sort, |x, y| x < y }
}

#[cfg(test)]
mod insertion_sort_tests {
    use super::insertion_sort;
    int_test_cases! { insertion_sort, |x, y| x < y }
}

#[cfg(test)]
mod bubble_sort_tests {
    use super::bubble_sort;
    int_test_cases! { bubble_sort, |x, y| x < y }
}