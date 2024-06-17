fn _merge_sort<T: Clone + Ord>(vec: &mut Vec<T>) {
    let len = vec.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = vec[..mid].to_vec();
    let mut right = vec[mid..].to_vec();

    _merge_sort(&mut left);
    _merge_sort(&mut right);
    _merge(vec, &left, &right);
}

fn _merge<T: Ord + Clone>(vec: &mut [T], left: &[T], right: &[T]) {
    let mut left_index = 0;
    let mut right_index = 0;
    let mut sorted_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            vec[sorted_index] = left[left_index].clone();
            left_index += 1;
        } else {
            vec[sorted_index] = right[right_index].clone();
            right_index += 1;
        }
        sorted_index += 1;
    }

    // while left_index < left.len() {
    //     vec[sorted_index] = left[left_index].clone();
    //     left_index += 1;
    //     sorted_index += 1;
    // }
    // 
    // while right_index < right.len() {
    //     vec[sorted_index] = right[right_index].clone();
    //     right_index += 1;
    //     sorted_index += 1;
    // }
    if left_index < left.len() {
        vec[sorted_index..].clone_from_slice(&left[left_index..]);
    }

    if right_index < right.len() {
        vec[sorted_index..].clone_from_slice(&right[right_index..]);
    }
}