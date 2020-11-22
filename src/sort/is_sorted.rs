pub fn is_sorted<T>(items: &[T], is_less: impl Fn(&T, &T) -> bool) -> bool {
    for i in 1..items.len() {
        if is_less(&items[i], &items[i - 1]) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::is_sorted;

    #[test]
    fn consecutive() {
        let vec: Vec<u32> = (0..10).collect();
        assert!(is_sorted(&vec, |x, y| x < y));
    }

    #[test]
    fn reverse_consecutive() {
        let vec: Vec<u32> = (0..10).rev().collect();
        assert!(!is_sorted(&vec, |x, y| x < y));
    }

    #[test]
    fn equal() {
        let vec: Vec<u32> = vec![0; 10];
        assert!(is_sorted(&vec, |x, y| x < y));
    }
}
