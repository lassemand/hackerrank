pub(crate) fn min_operations(a: &[i16]) -> i32 {
    let n = a.len();
    if n < 2 {
        return 0;
    }

    let mut min_val = a[0];
    for &v in &a[1..] {
        if v < min_val {
            min_val = v;
        }
    }

    let mut min_count = i32::MAX;

    for i in 0..=2 {
        let mut count: i32 = 0;

        for &val in a {
            let mut v = val - (min_val - i);
            count += (v / 5) as i32;
            v %= 5;
            count += (v / 2) as i32;
            count += (v & 1) as i32;
        }

        min_count = min_count.min(count);
    }

    min_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_element() {
        let a = vec![10];
        assert_eq!(min_operations(&a), 0);
    }

    #[test]
    fn test_already_equal() {
        let a = vec![5, 5, 5];
        assert_eq!(min_operations(&a), 0);
    }

    #[test]
    fn test_simple_case() {
        let a = vec![1, 2, 3];
        assert_eq!(min_operations(&a), 2);
    }

    #[test]
    fn test_hackerrank_sample() {
        let a = vec![2, 2, 3, 7];
        assert_eq!(min_operations(&a), 2);
    }

    #[test]
    fn test_large_gap() {
        let a = vec![1, 10];
        assert_eq!(min_operations(&a), 3);
    }
}