use crate::util;

pub fn selection_sort<T: Copy>(items: &mut [T], is_less: &impl Fn(&T, &T) -> bool) {
    for i in 1..items.len() {
        let mut min = i - 1;
        for j in i..items.len() {
            if is_less(&items[j], &items[min]) {
                min = j;
            }
        }
        if is_less(&items[min], &items[i - 1]) {
            util::swap(items, i - 1, min);
        }
    }
}

#[cfg(test)]
mod selection_sort_tests {
    use super::selection_sort;
    test! { selection_sort, &|x, y| x < y }
}
