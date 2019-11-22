use crate::util;

pub fn bubble_sort<T: Copy>(items: &mut [T], is_less: &impl Fn(&T, &T) -> bool) {
    for _ in 1..items.len() {
        for i in 1..items.len() {
            if is_less(&items[i], &items[i - 1]) {
                util::swap(items, i - 1, i);
            }
        }
    }
}

#[cfg(test)]
mod bubble_sort_tests {
    use super::bubble_sort;
    test! { bubble_sort, &|x, y| x < y }
}
