pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() == 0 {
        return;
    }
    let mut temp = vec![arr[0]; arr.len()];
    merge_sort_recursive(arr, &mut temp);
}

fn merge_sort_recursive<T: Ord + Copy>(arr: &mut [T], temp: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let (left, right) = arr.split_at_mut(mid);
    let (temp_left, temp_right) = temp.split_at_mut(mid);

    merge_sort_recursive(left, temp_left);
    merge_sort_recursive(right, temp_right);

    merge(left, right, temp);

    arr.copy_from_slice(temp);
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T], output: &mut [T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            output[k] = left[i];
            i += 1;
        } else {
            output[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    if i < left.len() {
        output[k..].copy_from_slice(&left[i..]);
    }

    if j < right.len() {
        output[k..].copy_from_slice(&right[j..]);
    }
}
