fn max_subarray(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return vec![0, 0];
    }

    let mut subsequence_max = 0;
    let mut max_element = arr[0];

    for &v in arr {
        if v > 0 {
            subsequence_max += v;
        }
        max_element = max_element.max(v);
    }

    if subsequence_max == 0 {
        subsequence_max = max_element;
    }

    let mut current_max = arr[0];
    let mut global_max = arr[0];

    for &v in &arr[1..] {
        current_max = (current_max + v).max(v);
        global_max = global_max.max(current_max);
    }

    vec![global_max, subsequence_max]
}

#[cfg(test)]
mod tests {
    use crate::maxsubarray::max_subarray;

    #[test]
    fn test_1() {
        let arr = vec![2, -1, 3];
        assert_eq!(max_subarray(&arr), vec![4, 5]);
    }

    #[test]
    fn test_2() {
        let arr = vec![2, -1, 3, -1, -1, -1, 4];
        assert_eq!(max_subarray(&arr), vec![5, 9]);
    }

    #[test]
    fn test_3() {
        let arr = vec![2, -1, 2, 3, 4, -5];
        assert_eq!(max_subarray(&arr), vec![10, 11]);
    }
}
