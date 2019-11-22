pub fn swap<T: Copy>(items: &mut [T], i: usize, j: usize) {
    let tmp = items[i];
    items[i] = items[j];
    items[j] = tmp;
}

pub fn subslice_copy_aligned<T: Copy>(src: &[T], dest: &mut [T], s: usize, e: usize) {
    for i in s..e {
        dest[i] = src[i];
    }
}
