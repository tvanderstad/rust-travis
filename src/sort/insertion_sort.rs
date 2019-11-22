use crate::util;

pub fn insertion_sort<T: Copy>(items: &mut [T], is_less: &impl Fn(&T, &T) -> bool) {
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
mod insertion_sort_tests {
    use super::insertion_sort;
    test! { insertion_sort, &|x, y| x < y }
}
