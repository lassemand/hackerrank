use std::cmp::max;

fn non_divisible_subset(k: usize, arr: &[i32]) -> i32 {
    let n = arr.len();
    if n == 0 {
        return 0;
    }
    let mut reminder_acc = vec![0; k];
    for v in arr {
        let i = v % k as i32;
        reminder_acc[i as usize] += 1
    }
    let mut acc = reminder_acc[0].min(1);
    for i in 1..=k/2 {
        let x = reminder_acc[i];
        let y = reminder_acc[k -i];

        let value = if i != k -i {
            x.max(y)
        } else {
            x.min(1)
        };
        acc += value;
    }
    acc
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(non_divisible_subset(3, &arr), 0);
    }

    #[test]
    fn test_single_element() {
        let arr = [7];
        assert_eq!(non_divisible_subset(3, &arr), 1);
    }

    #[test]
    fn test_example_case() {
        // Classic example
        let arr = [1, 7, 2, 4];
        let k = 3;
        // Valid subset: [1, 7, 4] → size 3
        assert_eq!(non_divisible_subset(k, &arr), 3);
    }

    #[test]
    fn test_all_divisible_pairs() {
        let arr = [3, 6, 9];
        let k = 3;
        // Only one element can be chosen
        assert_eq!(non_divisible_subset(k, &arr), 1);
    }

    #[test]
    fn test_no_conflicts() {
        let arr = [1, 2, 3, 4];
        let k = 9;
        // No pair sums to multiple of 7 → all included
        assert_eq!(non_divisible_subset(k, &arr), 4);
    }

    #[test]
    fn test_remainder_conflicts() {
        let arr = [1, 2, 3, 4, 5];
        let k = 4;
        // Optimal subset size is 3
        assert_eq!(non_divisible_subset(k, &arr), 4);
    }

    #[test]
    fn test_even_divisible() {
        let arr = [2, 2];
        let k = 4;
        // Optimal subset size is 3
        assert_eq!(non_divisible_subset(k, &arr), 1);
    }
}