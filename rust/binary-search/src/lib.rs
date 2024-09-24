pub fn find<T: Ord, V: AsRef<[T]>>(array: V, key: T) -> Option<usize> {
    let arr = array.as_ref();

    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if key == arr[mid] {
            return Some(mid);
        } else if key > arr[mid] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}
