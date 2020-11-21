pub fn is_sorted<T>(items: &[T], is_less: impl Fn(&T, &T) -> bool) -> bool {
    for i in 1..items.len() {
        if is_less(&items[i], &items[i - 1]) {
            return false;
        }
    }
    true
}
